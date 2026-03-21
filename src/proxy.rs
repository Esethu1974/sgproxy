use anyhow::{Result, anyhow};
use serde_json::{Value, json};
use url::Url;
use worker::{Fetch, Headers, Request, RequestInit, Response};

use crate::config::{
    ChannelKind, CredentialConfig, DEFAULT_ANTHROPIC_VERSION, DEFAULT_BASE_URL,
    DEFAULT_CODEX_CLIENT_VERSION, DEFAULT_CODEX_ORIGINATOR, DEFAULT_CODEX_USER_AGENT,
    DEFAULT_REQUIRED_BETA, DEFAULT_USER_AGENT,
};
use crate::oauth::codex_base_url;

pub struct ProxyOutcome {
    pub response: Response,
    pub status_code: u16,
}

pub async fn proxy_request(req: Request, credential: &CredentialConfig) -> Result<ProxyOutcome> {
    let codex_model_route = codex_model_route(&req, credential.channel)?;
    let reshape_codex_models = credential.channel == ChannelKind::Codex
        && !request_user_agent_contains_codex(req.headers())
        && !matches!(codex_model_route, CodexModelRoute::None);
    let upstream_url = build_upstream_url(&req, credential.channel)?;
    let headers = build_upstream_headers(req.headers(), credential)?;

    let mut init = RequestInit::new();
    init.with_method(req.method()).with_headers(headers);
    if let Some(body) = req.inner().body() {
        init.with_body(Some(body.into()));
    }

    let upstream_req = Request::new_with_init(upstream_url.as_str(), &init)?;
    let upstream_resp = Fetch::Request(upstream_req).send().await?;
    let status_code = upstream_resp.status_code();
    let response_headers = filter_response_headers(upstream_resp.headers())?;

    if reshape_codex_models {
        let mut upstream_resp = upstream_resp;
        let bytes = upstream_resp.bytes().await?;
        let response =
            reshape_codex_model_response(status_code, response_headers, &bytes, codex_model_route)?;
        return Ok(ProxyOutcome {
            response,
            status_code,
        });
    }

    let (_, body) = upstream_resp.into_parts();
    let response = Response::builder()
        .with_status(status_code)
        .with_headers(response_headers)
        .body(body);
    Ok(ProxyOutcome {
        response,
        status_code,
    })
}

fn build_upstream_url(req: &Request, channel: ChannelKind) -> Result<Url> {
    let source = req.url()?;
    let mut target = Url::parse(match channel {
        ChannelKind::ClaudeCode => DEFAULT_BASE_URL,
        ChannelKind::Codex => codex_base_url(),
    })?;
    let path = match channel {
        ChannelKind::ClaudeCode => source.path().to_string(),
        ChannelKind::Codex => {
            let path = source
                .path()
                .strip_prefix("/codex")
                .unwrap_or(source.path());
            if path.is_empty() {
                "/".to_string()
            } else {
                path.to_string()
            }
        }
    };
    target.set_path(&path);
    target.set_query(source.query());
    if channel == ChannelKind::Codex
        && path == "/models"
        && target
            .query()
            .is_none_or(|query| !query.contains("client_version="))
    {
        target
            .query_pairs_mut()
            .append_pair("client_version", DEFAULT_CODEX_CLIENT_VERSION);
    }
    Ok(target)
}

fn build_upstream_headers(original: &Headers, credential: &CredentialConfig) -> Result<Headers> {
    match credential.channel {
        ChannelKind::ClaudeCode => {
            build_claudecode_upstream_headers(original, credential.access_token.as_str())
        }
        ChannelKind::Codex => build_codex_upstream_headers(original, credential),
    }
}

fn build_claudecode_upstream_headers(original: &Headers, access_token: &str) -> Result<Headers> {
    let headers = Headers::new();
    let mut seen_user_agent = false;
    let mut seen_anthropic_version = false;
    let mut beta_values = Vec::new();

    for (name, value) in original.entries() {
        let lower = name.to_ascii_lowercase();
        if is_hop_by_hop(&lower)
            || matches!(
                lower.as_str(),
                "host" | "content-length" | "authorization" | "cookie"
            )
        {
            continue;
        }
        if lower == "anthropic-beta" {
            collect_beta_values(&value, &mut beta_values);
            continue;
        }
        if lower == "user-agent" {
            seen_user_agent = true;
        }
        if lower == "anthropic-version" {
            seen_anthropic_version = true;
        }
        headers.append(&name, &value)?;
    }

    collect_beta_values(DEFAULT_REQUIRED_BETA, &mut beta_values);
    headers.set("authorization", &format!("Bearer {access_token}"))?;
    if !seen_user_agent {
        headers.set("user-agent", DEFAULT_USER_AGENT)?;
    }
    if !seen_anthropic_version {
        headers.set("anthropic-version", DEFAULT_ANTHROPIC_VERSION)?;
    }
    if !beta_values.is_empty() {
        headers.set("anthropic-beta", &beta_values.join(","))?;
    }
    Ok(headers)
}

fn build_codex_upstream_headers(
    original: &Headers,
    credential: &CredentialConfig,
) -> Result<Headers> {
    let headers = Headers::new();
    let account_id = credential.account_id.as_deref().unwrap_or("").trim();
    if account_id.is_empty() {
        anyhow::bail!("missing account_id");
    }

    for (name, value) in original.entries() {
        let lower = name.to_ascii_lowercase();
        if is_hop_by_hop(&lower)
            || matches!(
                lower.as_str(),
                "host"
                    | "content-length"
                    | "authorization"
                    | "cookie"
                    | "chatgpt-account-id"
                    | "originator"
            )
        {
            continue;
        }
        headers.append(&name, &value)?;
    }

    headers.set(
        "authorization",
        &format!("Bearer {}", credential.access_token),
    )?;
    headers.set("chatgpt-account-id", account_id)?;
    headers.set("originator", DEFAULT_CODEX_ORIGINATOR)?;
    headers.set("user-agent", DEFAULT_CODEX_USER_AGENT)?;
    Ok(headers)
}

fn filter_response_headers(original: &Headers) -> Result<Headers> {
    let headers = Headers::new();
    for (name, value) in original.entries() {
        if is_hop_by_hop(&name) {
            continue;
        }
        headers.append(&name, &value)?;
    }
    Ok(headers)
}

fn collect_beta_values(raw: &str, target: &mut Vec<String>) {
    for value in raw
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        if !target.iter().any(|item| item == value) {
            target.push(value.to_string());
        }
    }
}

fn is_hop_by_hop(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "connection"
            | "keep-alive"
            | "proxy-authenticate"
            | "proxy-authorization"
            | "te"
            | "trailer"
            | "transfer-encoding"
            | "upgrade"
    )
}

#[derive(Debug, Clone)]
enum CodexModelRoute {
    None,
    List,
    Get(String),
}

fn codex_model_route(req: &Request, channel: ChannelKind) -> Result<CodexModelRoute> {
    if channel != ChannelKind::Codex || req.method() != worker::Method::Get {
        return Ok(CodexModelRoute::None);
    }
    let path = req.url()?.path().to_string();
    let path = path.strip_prefix("/codex").unwrap_or(&path);
    if path == "/models" {
        return Ok(CodexModelRoute::List);
    }
    if let Some(model_id) = path
        .strip_prefix("/models/")
        .filter(|value| !value.is_empty())
    {
        return Ok(CodexModelRoute::Get(model_id.to_string()));
    }
    Ok(CodexModelRoute::None)
}

fn request_user_agent_contains_codex(headers: &Headers) -> bool {
    headers
        .get("user-agent")
        .ok()
        .flatten()
        .map(|value| value.to_ascii_lowercase().contains("codex"))
        .unwrap_or(false)
}

fn reshape_codex_model_response(
    status_code: u16,
    headers: Headers,
    bytes: &[u8],
    route: CodexModelRoute,
) -> Result<Response> {
    let (response_status, body) = match route {
        CodexModelRoute::None => anyhow::bail!("invalid codex model route"),
        CodexModelRoute::List => build_openai_model_list_body(status_code, bytes)?,
        CodexModelRoute::Get(target) => build_openai_model_get_body(status_code, bytes, &target)?,
    };
    headers.delete("content-length")?;
    headers.delete("content-encoding")?;
    headers.set("content-type", "application/json")?;
    let response = Response::builder()
        .with_status(response_status)
        .with_headers(headers)
        .fixed(body.into_bytes());
    Ok(response)
}

fn build_openai_model_list_body(status_code: u16, bytes: &[u8]) -> Result<(u16, String)> {
    if status_code != 200 {
        return Ok((
            status_code,
            serde_json::to_string(&openai_model_error_body(
                extract_upstream_error_message(bytes)
                    .unwrap_or_else(|| format!("upstream status {status_code}")),
                None,
            ))?,
        ));
    }
    let parsed = serde_json::from_slice::<Value>(bytes)?;
    let normalized = normalize_openai_model_list_value(&parsed)
        .ok_or_else(|| anyhow!("invalid codex model-list payload"))?;
    Ok((status_code, serde_json::to_string(&normalized)?))
}

fn build_openai_model_get_body(
    status_code: u16,
    bytes: &[u8],
    target: &str,
) -> Result<(u16, String)> {
    if status_code != 200 {
        return Ok((
            status_code,
            serde_json::to_string(&openai_model_error_body(
                extract_upstream_error_message(bytes)
                    .unwrap_or_else(|| format!("upstream status {status_code}")),
                Some("model"),
            ))?,
        ));
    }
    let parsed = serde_json::from_slice::<Value>(bytes)?;
    let list = normalize_openai_model_list_value(&parsed)
        .ok_or_else(|| anyhow!("invalid codex model-list payload"))?;
    let Some(model) = find_model_in_openai_list(&list, target) else {
        return Ok((
            404,
            serde_json::to_string(&openai_model_error_body(
                format!("model {target} not found"),
                Some("model"),
            ))?,
        ));
    };
    Ok((status_code, serde_json::to_string(&model)?))
}

fn normalize_openai_model_list_value(value: &Value) -> Option<Value> {
    if is_openai_model_list(value) {
        return Some(value.clone());
    }
    let models = value.get("models")?.as_array()?;
    let mut data = Vec::new();
    for item in models {
        if let Some(model) = normalize_openai_model_value(item) {
            data.push(model);
        }
    }
    Some(json!({
        "object": "list",
        "data": data,
    }))
}

fn normalize_openai_model_value(value: &Value) -> Option<Value> {
    if is_openai_model_value(value) {
        return Some(value.clone());
    }
    let object = value.as_object()?;
    let id = object
        .get("id")
        .and_then(Value::as_str)
        .or_else(|| object.get("slug").and_then(Value::as_str))?;
    let created = object.get("created").and_then(Value::as_u64).unwrap_or(0);
    let owned_by = object
        .get("owned_by")
        .and_then(Value::as_str)
        .unwrap_or("openai");
    Some(json!({
        "id": normalize_model_id(id),
        "object": "model",
        "owned_by": owned_by,
        "created": created,
    }))
}

fn normalize_model_id(value: &str) -> String {
    value
        .trim()
        .strip_prefix("models/")
        .unwrap_or(value.trim())
        .to_string()
}

fn is_openai_model_list(value: &Value) -> bool {
    value.get("object").and_then(Value::as_str) == Some("list")
        && value.get("data").and_then(Value::as_array).is_some()
}

fn is_openai_model_value(value: &Value) -> bool {
    value.get("object").and_then(Value::as_str) == Some("model")
        && value.get("id").and_then(Value::as_str).is_some()
}

fn find_model_in_openai_list(list: &Value, target: &str) -> Option<Value> {
    let normalized_target = normalize_model_id(target);
    list.get("data")?
        .as_array()?
        .iter()
        .find(|item| {
            item.get("id")
                .and_then(Value::as_str)
                .map(|id| normalize_model_id(id) == normalized_target)
                .unwrap_or(false)
        })
        .cloned()
}

fn extract_upstream_error_message(bytes: &[u8]) -> Option<String> {
    let value = serde_json::from_slice::<Value>(bytes).ok()?;
    if let Some(message) = value
        .get("detail")
        .and_then(|detail| detail.get("message"))
        .and_then(Value::as_str)
    {
        return Some(message.to_string());
    }
    if let Some(message) = value
        .get("error")
        .and_then(|error| error.get("message"))
        .and_then(Value::as_str)
    {
        return Some(message.to_string());
    }
    if let Some(message) = value.get("error").and_then(Value::as_str) {
        return Some(message.to_string());
    }
    value
        .get("message")
        .and_then(Value::as_str)
        .map(ToString::to_string)
}

fn openai_model_error_body(message: String, param: Option<&str>) -> Value {
    json!({
        "error": {
            "message": message,
            "type": "invalid_request_error",
            "param": param,
            "code": "upstream_error",
        }
    })
}
