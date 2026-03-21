use anyhow::{Result, anyhow};
use base64::Engine as _;
use rand::Rng as _;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use sha2::{Digest as _, Sha256};
use url::form_urlencoded;
use wasm_bindgen::JsValue;
use worker::{Fetch, Headers, Method, Request, RequestInit};

use crate::config::{
    CLAUDE_CODE_OAUTH_CLIENT_ID, CLAUDE_CODE_OAUTH_SCOPE, CODEX_OAUTH_CLIENT_ID, ChannelKind,
    CodexUsageSnapshot, CodexUsageWindow, CredentialConfig, CredentialUsageBucket,
    CredentialUsageSnapshot, DEFAULT_ANTHROPIC_VERSION, DEFAULT_BASE_URL,
    DEFAULT_CLAUDE_AI_BASE_URL, DEFAULT_CODEX_BASE_URL, DEFAULT_CODEX_ISSUER,
    DEFAULT_CODEX_ORIGINATOR, DEFAULT_CODEX_REDIRECT_URI, DEFAULT_CODEX_SCOPE,
    DEFAULT_CODEX_USER_AGENT, DEFAULT_REDIRECT_URI, DEFAULT_REQUIRED_BETA,
    DEFAULT_TOKEN_USER_AGENT, DEFAULT_USER_AGENT, StoredOAuthState,
};
use crate::state::now_unix_ms;

#[derive(Debug, Clone, Deserialize)]
pub struct OAuthStartInput {
    #[serde(default)]
    pub redirect_uri: Option<String>,
    #[serde(default)]
    pub scope: Option<String>,
    #[serde(default)]
    pub issuer: Option<String>,
    #[serde(default)]
    pub originator: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OAuthCallbackInput {
    #[serde(default)]
    pub callback_url: Option<String>,
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OAuthStartResponse {
    pub auth_url: String,
    pub state: String,
    pub redirect_uri: String,
}

#[derive(Debug, Clone)]
pub struct OAuthStartState {
    pub response: OAuthStartResponse,
    pub stored_state: StoredOAuthState,
}

#[derive(Debug, Clone)]
pub struct RefreshedCredential {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at_unix_ms: u64,
    pub user_email: Option<String>,
    pub account_id: Option<String>,
    pub subscription_type: Option<String>,
    pub rate_limit_tier: Option<String>,
}

#[derive(Debug)]
pub enum RefreshError {
    InvalidCredential(String),
    Transient(String),
}

#[derive(Debug, Clone, Default)]
pub struct OAuthProfileParsed {
    pub email: Option<String>,
    pub account_id: Option<String>,
    pub subscription_type: Option<String>,
    pub rate_limit_tier: Option<String>,
    pub organization_uuid: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ClaudeTokenResponse {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_in: Option<u64>,
    #[serde(default, alias = "subscriptionType")]
    pub subscription_type: Option<String>,
    #[serde(default, alias = "rateLimitTier")]
    pub rate_limit_tier: Option<String>,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub error_description: Option<String>,
    #[serde(default, alias = "organizationUuid")]
    pub organization_uuid: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CodexTokenResponse {
    #[serde(default)]
    access_token: Option<String>,
    #[serde(default)]
    refresh_token: Option<String>,
    #[serde(default)]
    id_token: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_u64")]
    expires_in: Option<u64>,
    #[serde(default)]
    error: Option<Value>,
    #[serde(default)]
    error_description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OAuthProfile {
    #[serde(default)]
    account: OAuthProfileAccount,
    #[serde(default)]
    organization: OAuthProfileOrg,
}

#[derive(Debug, Default, Deserialize)]
struct OAuthProfileAccount {
    email: Option<String>,
    #[serde(default)]
    has_claude_max: bool,
    #[serde(default)]
    has_claude_pro: bool,
}

#[derive(Debug, Default, Deserialize)]
struct OAuthProfileOrg {
    uuid: Option<String>,
    organization_type: Option<String>,
    rate_limit_tier: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UsagePayload {
    #[serde(default)]
    five_hour: UsageBucketPayload,
    #[serde(default)]
    seven_day: UsageBucketPayload,
    #[serde(default)]
    seven_day_sonnet: UsageBucketPayload,
}

#[derive(Debug, Default, Deserialize)]
struct UsageBucketPayload {
    utilization: Option<f64>,
    resets_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CodexUsagePayload {
    #[serde(default, rename = "rate_limit")]
    rate_limit: Option<CodexRateLimitDetails>,
    #[serde(default)]
    credits: Option<CodexCreditsDetails>,
    #[serde(default)]
    plan_type: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CodexRateLimitDetails {
    #[serde(default)]
    primary_window: Option<CodexRateLimitWindowPayload>,
    #[serde(default)]
    secondary_window: Option<CodexRateLimitWindowPayload>,
}

#[derive(Debug, Deserialize)]
struct CodexRateLimitWindowPayload {
    #[serde(default)]
    used_percent: Option<i64>,
    #[serde(default)]
    limit_window_seconds: Option<i64>,
    #[serde(default, rename = "reset_at")]
    reset_at_unix_secs: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct CodexCreditsDetails {
    #[serde(default)]
    has_credits: Option<bool>,
    #[serde(default)]
    unlimited: Option<bool>,
    #[serde(default)]
    balance: Option<f64>,
}

#[derive(Debug, Default)]
struct CodexIdTokenClaims {
    email: Option<String>,
    plan: Option<String>,
    account_id: Option<String>,
}

enum U64Like {
    String(String),
    Number(u64),
}

impl<'de> Deserialize<'de> for U64Like {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Inner {
            String(String),
            Number(u64),
        }

        match Inner::deserialize(deserializer)? {
            Inner::String(value) => Ok(Self::String(value)),
            Inner::Number(value) => Ok(Self::Number(value)),
        }
    }
}

pub fn oauth_start_claudecode(input: OAuthStartInput) -> OAuthStartState {
    let redirect_uri = input
        .redirect_uri
        .and_then(clean_string)
        .unwrap_or_else(|| DEFAULT_REDIRECT_URI.to_string());
    let scope = input
        .scope
        .and_then(clean_string)
        .unwrap_or_else(|| CLAUDE_CODE_OAUTH_SCOPE.to_string());
    let state_id = generate_oauth_state();
    let code_verifier = generate_code_verifier(32);
    let code_challenge = generate_code_challenge(&code_verifier);
    let auth_url = build_claude_authorize_url(&redirect_uri, &scope, &code_challenge, &state_id);

    OAuthStartState {
        response: OAuthStartResponse {
            auth_url,
            state: state_id.clone(),
            redirect_uri: redirect_uri.clone(),
        },
        stored_state: StoredOAuthState {
            channel: ChannelKind::ClaudeCode,
            state_id,
            code_verifier,
            redirect_uri,
            oauth_issuer: None,
            created_at_unix_ms: now_unix_ms(),
        },
    }
}

pub fn oauth_start_codex(input: OAuthStartInput) -> OAuthStartState {
    let redirect_uri = input
        .redirect_uri
        .and_then(clean_string)
        .unwrap_or_else(|| DEFAULT_CODEX_REDIRECT_URI.to_string());
    let scope = input
        .scope
        .and_then(clean_string)
        .unwrap_or_else(|| DEFAULT_CODEX_SCOPE.to_string());
    let issuer = input
        .issuer
        .and_then(clean_string)
        .unwrap_or_else(|| DEFAULT_CODEX_ISSUER.to_string());
    let originator = input
        .originator
        .and_then(clean_string)
        .unwrap_or_else(|| DEFAULT_CODEX_ORIGINATOR.to_string());
    let state_id = generate_oauth_state();
    let code_verifier = generate_code_verifier(64);
    let code_challenge = generate_code_challenge(&code_verifier);
    let auth_url = build_codex_authorize_url(
        &issuer,
        &redirect_uri,
        &scope,
        &originator,
        &code_challenge,
        &state_id,
    );

    OAuthStartState {
        response: OAuthStartResponse {
            auth_url,
            state: state_id.clone(),
            redirect_uri: redirect_uri.clone(),
        },
        stored_state: StoredOAuthState {
            channel: ChannelKind::Codex,
            state_id,
            code_verifier,
            redirect_uri,
            oauth_issuer: Some(issuer),
            created_at_unix_ms: now_unix_ms(),
        },
    }
}

pub fn resolve_code_and_state(payload: &OAuthCallbackInput) -> Result<(String, Option<String>)> {
    let mut code = payload.code.clone().and_then(clean_string);
    let mut state = payload.state.clone().and_then(clean_string);

    if let Some(callback_url) = payload
        .callback_url
        .as_ref()
        .and_then(|value| clean_opt_str(value))
    {
        let callback_code = extract_value_from_text(&callback_url, "code");
        let callback_state = extract_value_from_text(&callback_url, "state");
        if code.is_none() {
            code = callback_code;
        }
        if state.is_none() {
            state = callback_state;
        }
        if code.is_none() {
            code = extract_manual_code(&callback_url);
        }
    }

    let code = code.ok_or_else(|| anyhow!("missing code"))?;
    Ok((code, state))
}

pub async fn exchange_claudecode_code_for_tokens(
    stored_state: &StoredOAuthState,
    code: &str,
) -> Result<ClaudeTokenResponse> {
    let cleaned_code = sanitize_oauth_code(code);
    let body = format!(
        "grant_type=authorization_code&client_id={}&code={}&redirect_uri={}&code_verifier={}&state={}",
        url_encode(CLAUDE_CODE_OAUTH_CLIENT_ID),
        url_encode(&cleaned_code),
        url_encode(&stored_state.redirect_uri),
        url_encode(&stored_state.code_verifier),
        url_encode(&stored_state.state_id),
    );

    let headers = default_claude_headers()?;
    headers.set("content-type", "application/x-www-form-urlencoded")?;
    headers.set("accept", "application/json, text/plain, */*")?;
    headers.set("origin", DEFAULT_CLAUDE_AI_BASE_URL)?;
    headers.set("referer", "https://claude.ai/")?;
    headers.set("user-agent", DEFAULT_TOKEN_USER_AGENT)?;

    let mut response = send_request(
        Method::Post,
        &format!("{}/v1/oauth/token", DEFAULT_BASE_URL),
        headers,
        Some(JsValue::from_str(&body)),
    )
    .await?;

    let status = response.status_code();
    let bytes = response.bytes().await?;
    if !(200..=299).contains(&status) {
        return Err(anyhow!(
            "oauth_token_failed: status={} body={}",
            status,
            String::from_utf8_lossy(&bytes)
        ));
    }
    Ok(serde_json::from_slice::<ClaudeTokenResponse>(&bytes)?)
}

pub async fn exchange_codex_code_for_tokens(
    stored_state: &StoredOAuthState,
    code: &str,
) -> Result<RefreshedCredential> {
    let issuer = stored_state
        .oauth_issuer
        .as_deref()
        .and_then(clean_opt_str)
        .unwrap_or_else(|| DEFAULT_CODEX_ISSUER.to_string());
    let body = format!(
        "grant_type=authorization_code&code={}&redirect_uri={}&client_id={}&code_verifier={}",
        url_encode(&sanitize_oauth_code(code)),
        url_encode(&stored_state.redirect_uri),
        url_encode(CODEX_OAUTH_CLIENT_ID),
        url_encode(&stored_state.code_verifier),
    );

    let headers = Headers::new();
    headers.set("content-type", "application/x-www-form-urlencoded")?;
    headers.set("accept", "application/json")?;
    headers.set("user-agent", DEFAULT_CODEX_USER_AGENT)?;

    let mut response = send_request(
        Method::Post,
        &format!("{}/oauth/token", issuer.trim_end_matches('/')),
        headers,
        Some(JsValue::from_str(&body)),
    )
    .await?;

    let status = response.status_code();
    let bytes = response.bytes().await?;
    if !(200..=299).contains(&status) {
        return Err(anyhow!(
            "oauth_token_failed: status={} body={}",
            status,
            String::from_utf8_lossy(&bytes)
        ));
    }

    let parsed = serde_json::from_slice::<CodexTokenResponse>(&bytes)?;
    codex_refreshed_from_token(parsed)
}

pub async fn maybe_refresh_access_token(
    credential: &CredentialConfig,
) -> Result<Option<RefreshedCredential>, RefreshError> {
    match credential.channel {
        ChannelKind::ClaudeCode => maybe_refresh_claudecode_access_token(credential).await,
        ChannelKind::Codex => maybe_refresh_codex_access_token(credential).await,
    }
}

pub async fn fetch_oauth_profile(access_token: &str) -> Result<OAuthProfileParsed> {
    let headers = Headers::new();
    headers.set("authorization", &format!("Bearer {access_token}"))?;
    headers.set("accept", "application/json")?;
    headers.set("anthropic-beta", DEFAULT_REQUIRED_BETA)?;
    headers.set("user-agent", DEFAULT_USER_AGENT)?;

    let mut response = send_request(
        Method::Get,
        &format!("{}/api/oauth/profile", DEFAULT_BASE_URL),
        headers,
        None,
    )
    .await?;
    let status = response.status_code();
    let bytes = response.bytes().await?;
    if !(200..=299).contains(&status) {
        return Err(anyhow!(
            "oauth_profile_failed: status={} body={}",
            status,
            String::from_utf8_lossy(&bytes)
        ));
    }
    let profile = serde_json::from_slice::<OAuthProfile>(&bytes)?;
    Ok(parse_profile(profile))
}

pub async fn fetch_claudecode_usage(access_token: &str) -> Result<CredentialUsageSnapshot> {
    let headers = Headers::new();
    headers.set("authorization", &format!("Bearer {access_token}"))?;
    headers.set("accept", "application/json")?;
    headers.set("anthropic-beta", DEFAULT_REQUIRED_BETA)?;
    headers.set("user-agent", DEFAULT_USER_AGENT)?;

    let mut response = send_request(
        Method::Get,
        &format!("{}/api/oauth/usage", DEFAULT_BASE_URL),
        headers,
        None,
    )
    .await?;
    let status = response.status_code();
    let bytes = response.bytes().await?;
    if !(200..=299).contains(&status) {
        return Err(anyhow!(
            "oauth_usage_failed: status={} body={}",
            status,
            String::from_utf8_lossy(&bytes)
        ));
    }

    let payload = serde_json::from_slice::<UsagePayload>(&bytes)?;
    Ok(CredentialUsageSnapshot {
        five_hour: parse_usage_bucket(payload.five_hour),
        seven_day: parse_usage_bucket(payload.seven_day),
        seven_day_sonnet: parse_usage_bucket(payload.seven_day_sonnet),
        codex: None,
        last_error: None,
    })
}

pub async fn fetch_codex_usage(
    access_token: &str,
    account_id: &str,
) -> Result<CredentialUsageSnapshot> {
    let headers = Headers::new();
    headers.set("authorization", &format!("Bearer {access_token}"))?;
    headers.set("chatgpt-account-id", account_id)?;
    headers.set("originator", DEFAULT_CODEX_ORIGINATOR)?;
    headers.set("user-agent", DEFAULT_CODEX_USER_AGENT)?;
    headers.set("accept", "application/json")?;

    let mut response = send_request(
        Method::Get,
        &format!(
            "{}/wham/usage",
            DEFAULT_CODEX_BASE_URL.trim_end_matches("/codex")
        ),
        headers,
        None,
    )
    .await?;
    let status = response.status_code();
    let bytes = response.bytes().await?;
    if !(200..=299).contains(&status) {
        return Err(anyhow!(
            "codex_usage_failed: status={} body={}",
            status,
            String::from_utf8_lossy(&bytes)
        ));
    }

    let payload = serde_json::from_slice::<CodexUsagePayload>(&bytes)?;
    let primary = payload
        .rate_limit
        .as_ref()
        .and_then(|item| item.primary_window.as_ref())
        .map(parse_codex_usage_window)
        .unwrap_or_default();
    let secondary = payload
        .rate_limit
        .as_ref()
        .and_then(|item| item.secondary_window.as_ref())
        .map(parse_codex_usage_window)
        .unwrap_or_default();

    Ok(CredentialUsageSnapshot {
        five_hour: CredentialUsageBucket::default(),
        seven_day: CredentialUsageBucket::default(),
        seven_day_sonnet: CredentialUsageBucket::default(),
        codex: Some(CodexUsageSnapshot {
            primary,
            secondary,
            plan_type: payload.plan_type.and_then(clean_string),
            credits_balance: payload.credits.as_ref().and_then(|item| item.balance),
            credits_unlimited: payload.credits.as_ref().and_then(|item| item.unlimited),
            has_credits: payload.credits.as_ref().and_then(|item| item.has_credits),
        }),
        last_error: None,
    })
}

async fn maybe_refresh_claudecode_access_token(
    credential: &CredentialConfig,
) -> Result<Option<RefreshedCredential>, RefreshError> {
    let now = now_unix_ms();
    if !credential.access_token.trim().is_empty()
        && credential.expires_at_unix_ms > now.saturating_add(60_000)
    {
        return Ok(None);
    }
    if credential.refresh_token.trim().is_empty() {
        return Err(RefreshError::InvalidCredential(
            "missing refresh_token".to_string(),
        ));
    }

    let body = format!(
        "grant_type=refresh_token&client_id={}&refresh_token={}",
        url_encode(CLAUDE_CODE_OAUTH_CLIENT_ID),
        url_encode(&credential.refresh_token),
    );

    let headers =
        default_claude_headers().map_err(|err| RefreshError::Transient(err.to_string()))?;
    headers
        .set("content-type", "application/x-www-form-urlencoded")
        .map_err(|err| RefreshError::Transient(err.to_string()))?;
    headers
        .set("accept", "application/json, text/plain, */*")
        .map_err(|err| RefreshError::Transient(err.to_string()))?;
    headers
        .set("user-agent", DEFAULT_TOKEN_USER_AGENT)
        .map_err(|err| RefreshError::Transient(err.to_string()))?;

    let mut response = send_request(
        Method::Post,
        &format!("{}/v1/oauth/token", DEFAULT_BASE_URL),
        headers,
        Some(JsValue::from_str(&body)),
    )
    .await
    .map_err(|err| RefreshError::Transient(err.to_string()))?;

    let status = response.status_code();
    let bytes = response
        .bytes()
        .await
        .map_err(|err| RefreshError::Transient(err.to_string()))?;
    let parsed = serde_json::from_slice::<ClaudeTokenResponse>(&bytes).ok();

    if (200..=299).contains(&status) {
        let access_token = parsed
            .as_ref()
            .and_then(|item| item.access_token.as_deref())
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| RefreshError::Transient("missing access_token".to_string()))?
            .to_string();
        let refresh_token = parsed
            .as_ref()
            .and_then(|item| item.refresh_token.as_deref())
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .unwrap_or(credential.refresh_token.as_str())
            .to_string();
        return Ok(Some(RefreshedCredential {
            access_token,
            refresh_token,
            expires_at_unix_ms: now.saturating_add(
                parsed
                    .as_ref()
                    .and_then(|item| item.expires_in)
                    .unwrap_or(3600)
                    .saturating_mul(1000),
            ),
            user_email: credential.user_email.clone(),
            account_id: credential.account_id.clone(),
            subscription_type: parsed
                .as_ref()
                .and_then(|item| item.subscription_type.clone()),
            rate_limit_tier: parsed
                .as_ref()
                .and_then(|item| item.rate_limit_tier.clone()),
        }));
    }

    let error = parsed
        .as_ref()
        .and_then(|item| item.error.as_deref())
        .unwrap_or_default();
    let description = parsed
        .as_ref()
        .and_then(|item| item.error_description.as_deref())
        .unwrap_or_default();
    let text = String::from_utf8_lossy(&bytes).to_string();
    let message = if error.is_empty() && description.is_empty() {
        format!("oauth token refresh failed: status={} body={text}", status)
    } else {
        format!(
            "oauth token refresh failed: status={} error={} description={}",
            status, error, description
        )
    };

    if status == 400 || status == 401 || status == 403 {
        Err(RefreshError::InvalidCredential(message))
    } else {
        Err(RefreshError::Transient(message))
    }
}

async fn maybe_refresh_codex_access_token(
    credential: &CredentialConfig,
) -> Result<Option<RefreshedCredential>, RefreshError> {
    let now = now_unix_ms();
    if !credential.access_token.trim().is_empty()
        && credential.expires_at_unix_ms > now.saturating_add(60_000)
    {
        return Ok(None);
    }
    if credential.refresh_token.trim().is_empty() {
        return Err(RefreshError::InvalidCredential(
            "missing refresh_token".to_string(),
        ));
    }

    let body = serde_json::to_string(&serde_json::json!({
        "client_id": CODEX_OAUTH_CLIENT_ID,
        "grant_type": "refresh_token",
        "refresh_token": credential.refresh_token,
        "scope": "openid profile email",
    }))
    .map_err(|err| RefreshError::Transient(err.to_string()))?;

    let headers = Headers::new();
    headers
        .set("content-type", "application/json")
        .map_err(|err| RefreshError::Transient(err.to_string()))?;
    headers
        .set("accept", "application/json")
        .map_err(|err| RefreshError::Transient(err.to_string()))?;
    headers
        .set("user-agent", DEFAULT_CODEX_USER_AGENT)
        .map_err(|err| RefreshError::Transient(err.to_string()))?;

    let mut response = send_request(
        Method::Post,
        &format!("{}/oauth/token", DEFAULT_CODEX_ISSUER),
        headers,
        Some(JsValue::from_str(&body)),
    )
    .await
    .map_err(|err| RefreshError::Transient(err.to_string()))?;

    let status = response.status_code();
    let bytes = response
        .bytes()
        .await
        .map_err(|err| RefreshError::Transient(err.to_string()))?;
    let parsed = serde_json::from_slice::<CodexTokenResponse>(&bytes).ok();

    if (200..=299).contains(&status) {
        let refreshed = parsed
            .map(codex_refreshed_from_token)
            .transpose()
            .map_err(|err| RefreshError::Transient(err.to_string()))?
            .ok_or_else(|| RefreshError::Transient("missing token response".to_string()))?;
        return Ok(Some(refreshed));
    }

    let message = codex_error_message(status, &bytes, parsed.as_ref());
    if status == 400 || status == 401 || status == 403 {
        Err(RefreshError::InvalidCredential(message))
    } else {
        Err(RefreshError::Transient(message))
    }
}

fn codex_refreshed_from_token(parsed: CodexTokenResponse) -> Result<RefreshedCredential> {
    let access_token = parsed
        .access_token
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| anyhow!("missing access_token"))?
        .to_string();
    let refresh_token = parsed
        .refresh_token
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| anyhow!("missing refresh_token"))?
        .to_string();
    let claims = parsed
        .id_token
        .as_deref()
        .map(parse_codex_id_token_claims)
        .unwrap_or_default();

    Ok(RefreshedCredential {
        access_token,
        refresh_token,
        expires_at_unix_ms: now_unix_ms()
            .saturating_add(parsed.expires_in.unwrap_or(3600).saturating_mul(1000)),
        user_email: claims.email,
        account_id: claims.account_id,
        subscription_type: claims.plan,
        rate_limit_tier: None,
    })
}

fn parse_profile(profile: OAuthProfile) -> OAuthProfileParsed {
    let subscription_type = profile
        .organization
        .organization_type
        .clone()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| {
            if profile.account.has_claude_max {
                Some("claude_max".to_string())
            } else if profile.account.has_claude_pro {
                Some("claude_pro".to_string())
            } else {
                None
            }
        });

    OAuthProfileParsed {
        email: profile.account.email,
        account_id: None,
        subscription_type,
        rate_limit_tier: profile.organization.rate_limit_tier,
        organization_uuid: profile.organization.uuid,
    }
}

fn parse_usage_bucket(bucket: UsageBucketPayload) -> CredentialUsageBucket {
    CredentialUsageBucket {
        utilization_pct: bucket
            .utilization
            .map(|value| value.round().clamp(0.0, 100.0) as u32),
        resets_at: bucket.resets_at.and_then(clean_string),
    }
}

fn parse_codex_usage_window(window: &CodexRateLimitWindowPayload) -> CodexUsageWindow {
    CodexUsageWindow {
        used_percent: window.used_percent.map(|value| value.clamp(0, 100) as u32),
        window_duration_mins: window
            .limit_window_seconds
            .map(|value| value.max(0) as u32 / 60),
        resets_at: window
            .reset_at_unix_secs
            .map(unix_secs_to_iso)
            .and_then(clean_string),
    }
}

fn default_claude_headers() -> Result<Headers> {
    let headers = Headers::new();
    headers.set("anthropic-version", DEFAULT_ANTHROPIC_VERSION)?;
    headers.set("anthropic-beta", DEFAULT_REQUIRED_BETA)?;
    Ok(headers)
}

async fn send_request(
    method: Method,
    url: &str,
    headers: Headers,
    body: Option<JsValue>,
) -> worker::Result<worker::Response> {
    let mut init = RequestInit::new();
    init.with_method(method)
        .with_headers(headers)
        .with_body(body);
    let request = Request::new_with_init(url, &init)?;
    Fetch::Request(request).send().await
}

fn build_claude_authorize_url(
    redirect_uri: &str,
    scope: &str,
    code_challenge: &str,
    state: &str,
) -> String {
    let query = vec![
        ("code".to_string(), "true".to_string()),
        (
            "client_id".to_string(),
            CLAUDE_CODE_OAUTH_CLIENT_ID.to_string(),
        ),
        ("response_type".to_string(), "code".to_string()),
        ("redirect_uri".to_string(), redirect_uri.to_string()),
        ("scope".to_string(), scope.to_string()),
        ("code_challenge".to_string(), code_challenge.to_string()),
        ("code_challenge_method".to_string(), "S256".to_string()),
        ("state".to_string(), state.to_string()),
    ]
    .into_iter()
    .map(|(key, value)| format!("{key}={}", url_encode(&value)))
    .collect::<Vec<_>>()
    .join("&");
    format!(
        "{}/oauth/authorize?{}",
        DEFAULT_CLAUDE_AI_BASE_URL.trim_end_matches('/'),
        query
    )
}

fn build_codex_authorize_url(
    issuer: &str,
    redirect_uri: &str,
    scope: &str,
    originator: &str,
    code_challenge: &str,
    state: &str,
) -> String {
    let query = vec![
        ("response_type".to_string(), "code".to_string()),
        ("client_id".to_string(), CODEX_OAUTH_CLIENT_ID.to_string()),
        ("redirect_uri".to_string(), redirect_uri.to_string()),
        ("scope".to_string(), scope.to_string()),
        ("code_challenge".to_string(), code_challenge.to_string()),
        ("code_challenge_method".to_string(), "S256".to_string()),
        ("id_token_add_organizations".to_string(), "true".to_string()),
        ("codex_cli_simplified_flow".to_string(), "true".to_string()),
        ("state".to_string(), state.to_string()),
        ("originator".to_string(), originator.to_string()),
    ]
    .into_iter()
    .map(|(key, value)| format!("{key}={}", url_encode(&value)))
    .collect::<Vec<_>>()
    .join("&");
    format!("{}/oauth/authorize?{query}", issuer.trim_end_matches('/'))
}

fn clean_opt_str(value: &str) -> Option<String> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

fn clean_string(value: String) -> Option<String> {
    clean_opt_str(&value)
}

fn generate_oauth_state() -> String {
    let mut bytes = [0u8; 24];
    rand::rng().fill_bytes(&mut bytes);
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

fn generate_code_verifier(len: usize) -> String {
    let mut bytes = vec![0u8; len];
    rand::rng().fill_bytes(&mut bytes);
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

fn generate_code_challenge(code_verifier: &str) -> String {
    let digest = Sha256::digest(code_verifier.as_bytes());
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(digest)
}

fn parse_query_value(raw: Option<&str>, key: &str) -> Option<String> {
    let raw = raw?.trim();
    let query = raw
        .split_once('?')
        .map(|(_, query)| query)
        .unwrap_or(raw)
        .trim_start_matches('?');
    for (name, value) in form_urlencoded::parse(query.as_bytes()) {
        if name == key {
            return Some(value.into_owned());
        }
    }
    None
}

fn extract_value_from_text(raw: &str, key: &str) -> Option<String> {
    parse_query_value(Some(raw), key)
        .or_else(|| parse_query_value(raw.split_once('#').map(|(_, fragment)| fragment), key))
        .or_else(|| extract_inline_query_value(raw, key))
        .or_else(|| {
            let decoded = percent_decode_lossy(raw);
            if decoded == raw {
                None
            } else {
                parse_query_value(Some(&decoded), key)
                    .or_else(|| extract_inline_query_value(&decoded, key))
            }
        })
}

fn extract_inline_query_value(raw: &str, key: &str) -> Option<String> {
    let needle = format!("{key}=");
    let index = raw.find(&needle)?;
    let start = index + needle.len();
    let rest = &raw[start..];
    let end = rest
        .find(['&', '#', '"', '\'', ' ', '\n', '\r', '\t'])
        .unwrap_or(rest.len());
    let value = rest[..end].trim();
    if value.is_empty() {
        return None;
    }
    Some(percent_decode_lossy(value))
}

fn extract_labeled_value(raw: &str, key: &str) -> Option<String> {
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let lower = trimmed.to_ascii_lowercase();
        for separator in [":", "="] {
            let prefix = format!("{key}{separator}");
            if lower.starts_with(&prefix) {
                let value = trimmed[prefix.len()..].trim();
                if !value.is_empty() {
                    return Some(value.to_string());
                }
            }
        }
    }
    None
}

fn extract_manual_code(raw: &str) -> Option<String> {
    if let Some(code) = extract_labeled_value(raw, "code") {
        return Some(code);
    }

    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    let looks_structured = trimmed.contains("://")
        || trimmed.contains('?')
        || trimmed.contains('&')
        || trimmed.contains("code=")
        || trimmed.contains("state=");
    if looks_structured {
        return None;
    }
    (!trimmed.contains(char::is_whitespace)).then(|| trimmed.to_string())
}

fn percent_decode_lossy(value: &str) -> String {
    form_urlencoded::parse(format!("x={value}").as_bytes())
        .next()
        .map(|(_, decoded)| decoded.into_owned())
        .unwrap_or_else(|| value.to_string())
}

fn url_encode(value: &str) -> String {
    form_urlencoded::byte_serialize(value.as_bytes()).collect::<String>()
}

fn unix_secs_to_iso(value: i64) -> String {
    js_sys::Date::new(&JsValue::from_f64((value.max(0) as f64) * 1000.0))
        .to_iso_string()
        .into()
}

fn sanitize_oauth_code(code: &str) -> String {
    let code = code.split('#').next().unwrap_or(code);
    let code = code.split('&').next().unwrap_or(code);
    code.trim().to_string()
}

fn parse_codex_id_token_claims(id_token: &str) -> CodexIdTokenClaims {
    let mut claims = CodexIdTokenClaims::default();
    let mut parts = id_token.split('.');
    let (_header, payload_b64, _signature) = match (parts.next(), parts.next(), parts.next()) {
        (Some(header), Some(payload), Some(signature))
            if !header.is_empty() && !payload.is_empty() && !signature.is_empty() =>
        {
            (header, payload, signature)
        }
        _ => return claims,
    };

    let payload_bytes = match base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(payload_b64) {
        Ok(bytes) => bytes,
        Err(_) => return claims,
    };
    let payload = match serde_json::from_slice::<Value>(&payload_bytes) {
        Ok(value) => value,
        Err(_) => return claims,
    };

    claims.email = payload
        .get("email")
        .and_then(Value::as_str)
        .or_else(|| {
            payload
                .get("https://api.openai.com/profile")
                .and_then(|profile| profile.get("email"))
                .and_then(Value::as_str)
        })
        .map(ToString::to_string);

    if let Some(auth) = payload.get("https://api.openai.com/auth") {
        claims.plan = auth
            .get("chatgpt_plan_type")
            .and_then(Value::as_str)
            .map(ToString::to_string);
        claims.account_id = auth
            .get("chatgpt_account_id")
            .and_then(Value::as_str)
            .map(ToString::to_string);
    }

    claims
}

fn deserialize_optional_u64<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<U64Like>::deserialize(deserializer)?;
    value
        .map(|item| match item {
            U64Like::String(value) => value
                .trim()
                .parse::<u64>()
                .map_err(serde::de::Error::custom),
            U64Like::Number(value) => Ok(value),
        })
        .transpose()
}

fn codex_error_message(status: u16, bytes: &[u8], parsed: Option<&CodexTokenResponse>) -> String {
    let description = parsed
        .and_then(|item| item.error_description.as_deref())
        .unwrap_or_default();
    let detail = parsed
        .and_then(|item| item.error.as_ref())
        .map(stringify_codex_error)
        .unwrap_or_default();
    let suffix = [detail, description.to_string()]
        .into_iter()
        .filter(|item| !item.trim().is_empty())
        .collect::<Vec<_>>()
        .join(" | ");
    if suffix.is_empty() {
        format!(
            "oauth token refresh failed: status={} body={}",
            status,
            String::from_utf8_lossy(bytes)
        )
    } else {
        format!("oauth token refresh failed: status={} {}", status, suffix)
    }
}

fn stringify_codex_error(error: &Value) -> String {
    match error {
        Value::Object(map) => {
            let error_type = map
                .get("type")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .trim()
                .to_string();
            let message = map
                .get("message")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .trim()
                .to_string();
            let code = map
                .get("code")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .trim()
                .to_string();
            [
                error_type,
                message,
                (!code.is_empty())
                    .then(|| format!("code={code}"))
                    .unwrap_or_default(),
            ]
            .into_iter()
            .filter(|item| !item.is_empty())
            .collect::<Vec<_>>()
            .join(" | ")
        }
        Value::String(value) => value.clone(),
        other => other.to_string(),
    }
}

pub fn parse_codex_import_profile(id_token: Option<&str>) -> OAuthProfileParsed {
    let claims = id_token
        .map(parse_codex_id_token_claims)
        .unwrap_or_default();
    OAuthProfileParsed {
        email: claims.email,
        account_id: claims.account_id,
        subscription_type: claims.plan,
        rate_limit_tier: None,
        organization_uuid: None,
    }
}

pub fn codex_base_url() -> &'static str {
    DEFAULT_CODEX_BASE_URL
}
