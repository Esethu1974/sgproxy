use anyhow::{Result, anyhow};
use serde_json::json;
use worker::{Env, Headers, Method, Request, Response, State, durable_object};

use crate::config::{
    ChannelKind, CredentialConfig, CredentialStatus, CredentialUpsertInput,
    CredentialUsageSnapshot, DurableStateDoc,
};
use crate::oauth::{
    OAuthCallbackInput, OAuthStartInput, RefreshError, exchange_claudecode_code_for_tokens,
    exchange_codex_code_for_tokens, fetch_claudecode_usage, fetch_codex_usage, fetch_oauth_profile,
    maybe_refresh_access_token, oauth_start_claudecode, oauth_start_codex,
    parse_codex_import_profile, resolve_code_and_state,
};
use crate::proxy::proxy_request;
use crate::state::{
    build_usage_view, delete_credential, first_usable, insert_oauth_state, load_doc, now_unix_ms,
    record_invalid_auth, record_rate_limited, record_success, record_transient, save_doc,
    set_enabled, take_oauth_state, upsert_credential,
};
use crate::tokenizer::count_openai_response_input_tokens;

#[durable_object]
pub struct SgproxyState {
    state: State,
    env: Env,
}

impl worker::DurableObject for SgproxyState {
    fn new(state: State, env: Env) -> Self {
        Self { state, env }
    }

    async fn fetch(&self, req: Request) -> worker::Result<Response> {
        let method = req.method();
        let path = req.path();

        let response = match self.route(method, path.as_str(), req).await {
            Ok(response) => response,
            Err(err) => json_error(400, &err.to_string())?,
        };
        Ok(response)
    }
}

impl SgproxyState {
    async fn route(&self, method: Method, path: &str, req: Request) -> Result<Response> {
        if path == "/api/claudecode/public/credentials" && method == Method::Get {
            return self.public_credentials(ChannelKind::ClaudeCode).await;
        }

        if let Some(tail) = path.strip_prefix("/api/claudecode") {
            return self
                .route_api(ChannelKind::ClaudeCode, method, tail, req)
                .await;
        }
        if let Some(tail) = path.strip_prefix("/api/codex") {
            return self.route_api(ChannelKind::Codex, method, tail, req).await;
        }
        if path == "/v1" || path.starts_with("/v1/") {
            return self.proxy(req, ChannelKind::ClaudeCode).await;
        }
        if method == Method::Post && path == "/codex/v1/responses/input_tokens" {
            return self.codex_input_tokens(req).await;
        }
        if path == "/codex" || path.starts_with("/codex/") {
            return self.proxy(req, ChannelKind::Codex).await;
        }

        json_error(404, "not found").map_err(Into::into)
    }

    async fn route_api(
        &self,
        channel: ChannelKind,
        method: Method,
        tail: &str,
        mut req: Request,
    ) -> Result<Response> {
        if tail == "/credentials" && method == Method::Get {
            self.authorize(&req)?;
            return self.list_credentials(channel).await;
        }
        if tail == "/credentials" && method == Method::Post {
            self.authorize(&req)?;
            let payload = req.json::<CredentialUpsertInput>().await?;
            return self.create_credential(channel, payload).await;
        }
        if tail == "/credentials/usage" && method == Method::Get {
            self.authorize(&req)?;
            return self.list_usage(channel).await;
        }
        if method == Method::Get && tail.starts_with("/credentials/usage/") {
            self.authorize(&req)?;
            return self
                .get_usage(channel, tail.trim_start_matches("/credentials/usage/"))
                .await;
        }
        if tail == "/oauth/start" && method == Method::Post {
            self.authorize(&req)?;
            let payload = req.json::<OAuthStartInput>().await?;
            return self.oauth_start(channel, payload).await;
        }
        if tail == "/oauth/callback" && method == Method::Post {
            self.authorize(&req)?;
            let payload = req.json::<OAuthCallbackInput>().await?;
            return self.oauth_callback(channel, payload).await;
        }
        if let Some(id) = tail.strip_prefix("/credentials/") {
            if id.ends_with("/enable") && method == Method::Post {
                self.authorize(&req)?;
                return self
                    .set_enabled(channel, id.trim_end_matches("/enable"), true)
                    .await;
            }
            if id.ends_with("/disable") && method == Method::Post {
                self.authorize(&req)?;
                return self
                    .set_enabled(channel, id.trim_end_matches("/disable"), false)
                    .await;
            }
            if !id.contains('/') && method == Method::Put {
                self.authorize(&req)?;
                let payload = req.json::<CredentialUpsertInput>().await?;
                return self.update_credential(channel, id, payload).await;
            }
            if !id.contains('/') && method == Method::Delete {
                self.authorize(&req)?;
                return self.delete_credential(channel, id).await;
            }
        }
        json_error(404, "not found").map_err(Into::into)
    }

    fn authorize(&self, req: &Request) -> Result<()> {
        let expected = self
            .env
            .secret("ADMIN_TOKEN")
            .map(|value| value.to_string())
            .or_else(|_| self.env.var("ADMIN_TOKEN").map(|value| value.to_string()))
            .map_err(|_| anyhow!("missing ADMIN_TOKEN secret"))?;
        let provided = bearer_token(req.headers()).ok_or_else(|| anyhow!("unauthorized"))?;
        if provided == expected {
            Ok(())
        } else {
            Err(anyhow!("unauthorized"))
        }
    }

    async fn public_credentials(&self, channel: ChannelKind) -> Result<Response> {
        let doc = load_doc(&self.state.storage()).await?;
        let response = self.build_usage_payload(channel, doc.credentials).await;
        Ok(Response::from_json(&response)?)
    }

    async fn list_usage(&self, channel: ChannelKind) -> Result<Response> {
        let doc = load_doc(&self.state.storage()).await?;
        let response = self.build_usage_payload(channel, doc.credentials).await;
        Ok(Response::from_json(&response)?)
    }

    async fn get_usage(&self, channel: ChannelKind, id: &str) -> Result<Response> {
        let doc = load_doc(&self.state.storage()).await?;
        let credential = doc
            .credentials
            .into_iter()
            .find(|item| item.channel == channel && item.id == id)
            .ok_or_else(|| anyhow!("credential not found: {id}"))?;
        let mut items = self.build_usage_payload(channel, vec![credential]).await;
        let view = items
            .pop()
            .ok_or_else(|| anyhow!("credential not found: {id}"))?;
        Ok(Response::from_json(&view)?)
    }

    async fn list_credentials(&self, channel: ChannelKind) -> Result<Response> {
        let mut doc = load_doc(&self.state.storage()).await?;
        doc.normalize(now_unix_ms());
        save_doc(&self.state.storage(), &doc).await?;
        let items = doc
            .credentials
            .into_iter()
            .filter(|item| item.channel == channel)
            .collect::<Vec<_>>();
        Ok(Response::from_json(&items)?)
    }

    async fn create_credential(
        &self,
        channel: ChannelKind,
        payload: CredentialUpsertInput,
    ) -> Result<Response> {
        let mut doc = load_doc(&self.state.storage()).await?;
        let resolved = self
            .resolve_credential_input(channel, payload, None, &doc)
            .await?;
        let credential = upsert_credential(&mut doc, resolved, None, channel);
        doc.normalize(now_unix_ms());
        save_doc(&self.state.storage(), &doc).await?;
        Ok(Response::from_json(&credential)?)
    }

    async fn update_credential(
        &self,
        channel: ChannelKind,
        id: &str,
        payload: CredentialUpsertInput,
    ) -> Result<Response> {
        let mut doc = load_doc(&self.state.storage()).await?;
        let existing = doc
            .credentials
            .iter()
            .find(|item| item.channel == channel && item.id == id)
            .cloned();
        if existing.is_none() {
            return Err(anyhow!("credential not found: {id}"));
        }
        let resolved = self
            .resolve_credential_input(channel, payload, Some(id), &doc)
            .await?;
        let credential = upsert_credential(&mut doc, resolved, Some(id), channel);
        doc.normalize(now_unix_ms());
        save_doc(&self.state.storage(), &doc).await?;
        Ok(Response::from_json(&credential)?)
    }

    async fn set_enabled(&self, channel: ChannelKind, id: &str, enabled: bool) -> Result<Response> {
        let mut doc = load_doc(&self.state.storage()).await?;
        ensure_channel_credential(&doc, channel, id)?;
        let credential = set_enabled(&mut doc, id, enabled)?;
        save_doc(&self.state.storage(), &doc).await?;
        Ok(Response::from_json(&credential)?)
    }

    async fn delete_credential(&self, channel: ChannelKind, id: &str) -> Result<Response> {
        let mut doc = load_doc(&self.state.storage()).await?;
        ensure_channel_credential(&doc, channel, id)?;
        delete_credential(&mut doc, id)?;
        save_doc(&self.state.storage(), &doc).await?;
        Ok(Response::from_json(&json!({ "ok": true }))?)
    }

    async fn oauth_start(
        &self,
        channel: ChannelKind,
        payload: OAuthStartInput,
    ) -> Result<Response> {
        let started = match channel {
            ChannelKind::ClaudeCode => oauth_start_claudecode(payload),
            ChannelKind::Codex => oauth_start_codex(payload),
        };
        let mut doc = load_doc(&self.state.storage()).await?;
        insert_oauth_state(&mut doc, started.stored_state);
        save_doc(&self.state.storage(), &doc).await?;
        Ok(Response::from_json(&started.response)?)
    }

    async fn oauth_callback(
        &self,
        channel: ChannelKind,
        payload: OAuthCallbackInput,
    ) -> Result<Response> {
        let (code, requested_state) = resolve_code_and_state(&payload)?;
        let mut doc = load_doc(&self.state.storage()).await?;
        let oauth_state = take_oauth_state(&mut doc, channel, requested_state.as_deref())?;

        let credential = match channel {
            ChannelKind::ClaudeCode => {
                let token = exchange_claudecode_code_for_tokens(&oauth_state, &code).await?;
                let access_token = token
                    .access_token
                    .as_deref()
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .ok_or_else(|| anyhow!("missing_access_token"))?
                    .to_string();
                let refresh_token = token
                    .refresh_token
                    .as_deref()
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .ok_or_else(|| anyhow!("missing_refresh_token"))?
                    .to_string();
                let profile = fetch_oauth_profile(&access_token).await.ok();
                let input = CredentialUpsertInput {
                    id: None,
                    channel: Some(channel),
                    enabled: Some(true),
                    order: None,
                    access_token: Some(access_token),
                    refresh_token: Some(refresh_token),
                    id_token: None,
                    expires_at_unix_ms: Some(
                        now_unix_ms()
                            .saturating_add(token.expires_in.unwrap_or(3600).saturating_mul(1000)),
                    ),
                    user_email: profile.as_ref().and_then(|item| item.email.clone()),
                    account_id: None,
                    organization_uuid: token.organization_uuid.clone().or_else(|| {
                        profile
                            .as_ref()
                            .and_then(|item| item.organization_uuid.clone())
                    }),
                    subscription_type: token.subscription_type.clone().or_else(|| {
                        profile
                            .as_ref()
                            .and_then(|item| item.subscription_type.clone())
                    }),
                    rate_limit_tier: token.rate_limit_tier.clone().or_else(|| {
                        profile
                            .as_ref()
                            .and_then(|item| item.rate_limit_tier.clone())
                    }),
                };
                upsert_credential(&mut doc, input, None, channel)
            }
            ChannelKind::Codex => {
                let token = exchange_codex_code_for_tokens(&oauth_state, &code).await?;
                let input = CredentialUpsertInput {
                    id: None,
                    channel: Some(channel),
                    enabled: Some(true),
                    order: None,
                    access_token: Some(token.access_token),
                    refresh_token: Some(token.refresh_token),
                    id_token: None,
                    expires_at_unix_ms: Some(token.expires_at_unix_ms),
                    user_email: token.user_email,
                    account_id: token.account_id,
                    organization_uuid: None,
                    subscription_type: token.subscription_type,
                    rate_limit_tier: token.rate_limit_tier,
                };
                upsert_credential(&mut doc, input, None, channel)
            }
        };

        doc.normalize(now_unix_ms());
        save_doc(&self.state.storage(), &doc).await?;
        Ok(Response::from_json(&json!({ "credential": credential }))?)
    }

    async fn proxy(&self, req: Request, channel: ChannelKind) -> Result<Response> {
        let mut doc = load_doc(&self.state.storage()).await?;
        let selected = self.resolve_proxy_credential(channel, &mut doc).await?;
        save_doc(&self.state.storage(), &doc).await?;
        let is_codex_review = channel == ChannelKind::Codex && request_subagent_is_review(&req);

        let result = proxy_request(req, &selected).await;
        let now = now_unix_ms();
        let mut doc = load_doc(&self.state.storage()).await?;

        match result {
            Ok(outcome) => {
                match outcome.status_code {
                    200..=299 => record_success(&mut doc, &selected.id, now),
                    401 | 403 => {
                        record_invalid_auth(
                            &mut doc,
                            &selected.id,
                            now,
                            format!("upstream returned status {}", outcome.status_code),
                        );
                    }
                    429 => match channel {
                        ChannelKind::ClaudeCode => {
                            match fetch_claudecode_usage(&selected.access_token).await {
                                Ok(usage) => {
                                    record_rate_limited(
                                        &mut doc,
                                        &selected.id,
                                        now,
                                        Some(&usage),
                                        None,
                                        false,
                                    );
                                }
                                Err(err) => {
                                    record_rate_limited(
                                        &mut doc,
                                        &selected.id,
                                        now,
                                        None,
                                        Some(format!(
                                            "upstream returned status 429; usage fetch failed: {err}"
                                        )),
                                        false,
                                    );
                                }
                            }
                        }
                        ChannelKind::Codex => match selected.account_id.as_deref() {
                            Some(account_id) if !account_id.trim().is_empty() => {
                                match fetch_codex_usage(&selected.access_token, account_id).await {
                                    Ok(usage) => {
                                        record_rate_limited(
                                            &mut doc,
                                            &selected.id,
                                            now,
                                            Some(&usage),
                                            None,
                                            is_codex_review,
                                        );
                                    }
                                    Err(err) => {
                                        record_rate_limited(
                                            &mut doc,
                                            &selected.id,
                                            now,
                                            None,
                                            Some(format!(
                                                "upstream returned status 429; usage fetch failed: {err}"
                                            )),
                                            is_codex_review,
                                        );
                                    }
                                }
                            }
                            _ => {
                                record_rate_limited(
                                    &mut doc,
                                    &selected.id,
                                    now,
                                    None,
                                    Some(
                                        "upstream returned status 429; missing account_id"
                                            .to_string(),
                                    ),
                                    is_codex_review,
                                );
                            }
                        },
                    },
                    status => {
                        record_transient(
                            &mut doc,
                            &selected.id,
                            now,
                            format!("upstream returned status {status}"),
                        );
                    }
                }
                save_doc(&self.state.storage(), &doc).await?;
                Ok(outcome.response)
            }
            Err(err) => {
                record_transient(&mut doc, &selected.id, now, err.to_string());
                save_doc(&self.state.storage(), &doc).await?;
                json_error(502, &err.to_string()).map_err(Into::into)
            }
        }
    }

    async fn codex_input_tokens(&self, mut req: Request) -> Result<Response> {
        let body = req.json::<serde_json::Value>().await?;
        let response = count_openai_response_input_tokens(&body)?;
        Ok(Response::from_json(&response)?)
    }

    async fn resolve_proxy_credential(
        &self,
        channel: ChannelKind,
        doc: &mut DurableStateDoc,
    ) -> Result<CredentialConfig> {
        loop {
            doc.normalize(now_unix_ms());
            let selected = first_usable(&doc.credentials, channel, now_unix_ms())
                .ok_or_else(|| anyhow!("no usable credential configured"))?;

            match maybe_refresh_access_token(&selected).await {
                Ok(Some(refreshed)) => {
                    let updated = upsert_credential(
                        doc,
                        CredentialUpsertInput {
                            id: Some(selected.id.clone()),
                            channel: Some(channel),
                            enabled: Some(selected.enabled),
                            order: Some(selected.order),
                            access_token: Some(refreshed.access_token),
                            refresh_token: Some(refreshed.refresh_token),
                            id_token: None,
                            expires_at_unix_ms: Some(refreshed.expires_at_unix_ms),
                            user_email: refreshed.user_email.or(selected.user_email.clone()),
                            account_id: refreshed.account_id.or(selected.account_id.clone()),
                            organization_uuid: selected.organization_uuid.clone(),
                            subscription_type: refreshed
                                .subscription_type
                                .or_else(|| selected.subscription_type.clone()),
                            rate_limit_tier: refreshed
                                .rate_limit_tier
                                .or_else(|| selected.rate_limit_tier.clone()),
                        },
                        Some(&selected.id),
                        channel,
                    );
                    return Ok(updated);
                }
                Ok(None) => return Ok(selected),
                Err(RefreshError::InvalidCredential(message)) => {
                    record_invalid_auth(doc, &selected.id, now_unix_ms(), message);
                }
                Err(RefreshError::Transient(message)) => return Err(anyhow!(message)),
            }
        }
    }

    async fn resolve_credential_input(
        &self,
        channel: ChannelKind,
        input: CredentialUpsertInput,
        forced_id: Option<&str>,
        doc: &DurableStateDoc,
    ) -> Result<CredentialUpsertInput> {
        let existing = forced_id
            .and_then(|id| {
                doc.credentials
                    .iter()
                    .find(|item| item.channel == channel && item.id == id)
            })
            .cloned()
            .or_else(|| {
                input.id.as_deref().and_then(|id| {
                    doc.credentials
                        .iter()
                        .find(|item| item.channel == channel && item.id == id)
                        .cloned()
                })
            });

        let import_profile = match channel {
            ChannelKind::ClaudeCode => None,
            ChannelKind::Codex => Some(parse_codex_import_profile(input.id_token.as_deref())),
        };

        let mut access_token = input
            .access_token
            .clone()
            .and_then(|value| crate::config::clean_opt_owned(Some(value)))
            .or_else(|| {
                existing
                    .as_ref()
                    .map(|item| item.access_token.clone())
                    .filter(|value| !value.is_empty())
            });
        let mut refresh_token = input
            .refresh_token
            .clone()
            .and_then(|value| crate::config::clean_opt_owned(Some(value)))
            .or_else(|| {
                existing
                    .as_ref()
                    .map(|item| item.refresh_token.clone())
                    .filter(|value| !value.is_empty())
            });
        let mut expires_at_unix_ms = input
            .expires_at_unix_ms
            .or_else(|| existing.as_ref().map(|item| item.expires_at_unix_ms));
        let mut user_email = input
            .user_email
            .clone()
            .and_then(|value| crate::config::clean_opt_owned(Some(value)))
            .or_else(|| existing.as_ref().and_then(|item| item.user_email.clone()))
            .or_else(|| import_profile.as_ref().and_then(|item| item.email.clone()));
        let mut account_id = input
            .account_id
            .clone()
            .and_then(|value| crate::config::clean_opt_owned(Some(value)))
            .or_else(|| existing.as_ref().and_then(|item| item.account_id.clone()))
            .or_else(|| {
                import_profile
                    .as_ref()
                    .and_then(|item| item.account_id.clone())
            });
        let mut organization_uuid = input
            .organization_uuid
            .clone()
            .and_then(|value| crate::config::clean_opt_owned(Some(value)))
            .or_else(|| {
                existing
                    .as_ref()
                    .and_then(|item| item.organization_uuid.clone())
            });
        let mut subscription_type = input
            .subscription_type
            .clone()
            .and_then(|value| crate::config::clean_opt_owned(Some(value)))
            .or_else(|| {
                existing
                    .as_ref()
                    .and_then(|item| item.subscription_type.clone())
            })
            .or_else(|| {
                import_profile
                    .as_ref()
                    .and_then(|item| item.subscription_type.clone())
            });
        let mut rate_limit_tier = input
            .rate_limit_tier
            .clone()
            .and_then(|value| crate::config::clean_opt_owned(Some(value)))
            .or_else(|| {
                existing
                    .as_ref()
                    .and_then(|item| item.rate_limit_tier.clone())
            });

        if access_token.is_none() && refresh_token.is_none() {
            return Err(anyhow!("missing access_token or refresh_token"));
        }

        if let Some(refresh) = refresh_token.clone()
            && (access_token.is_none() || expires_at_unix_ms.unwrap_or(0) <= now_unix_ms())
        {
            let refreshed = maybe_refresh_access_token(&CredentialConfig {
                id: existing
                    .as_ref()
                    .map(|item| item.id.clone())
                    .unwrap_or_else(|| "import".to_string()),
                channel,
                enabled: existing.as_ref().map(|item| item.enabled).unwrap_or(true),
                order: existing.as_ref().map(|item| item.order).unwrap_or(0),
                access_token: access_token.clone().unwrap_or_default(),
                refresh_token: refresh,
                expires_at_unix_ms: expires_at_unix_ms.unwrap_or(0),
                user_email: user_email.clone(),
                account_id: account_id.clone(),
                organization_uuid: organization_uuid.clone(),
                subscription_type: subscription_type.clone(),
                rate_limit_tier: rate_limit_tier.clone(),
                status: CredentialStatus::Healthy,
                cooldown_until_unix_ms: None,
                last_error: None,
                last_used_at_unix_ms: None,
            })
            .await
            .map_err(|err| match err {
                RefreshError::InvalidCredential(message) | RefreshError::Transient(message) => {
                    anyhow!(message)
                }
            })?;
            if let Some(refreshed) = refreshed {
                access_token = Some(refreshed.access_token);
                refresh_token = Some(refreshed.refresh_token);
                expires_at_unix_ms = Some(refreshed.expires_at_unix_ms);
                if user_email.is_none() {
                    user_email = refreshed.user_email;
                }
                if account_id.is_none() {
                    account_id = refreshed.account_id;
                }
                if subscription_type.is_none() {
                    subscription_type = refreshed.subscription_type;
                }
                if rate_limit_tier.is_none() {
                    rate_limit_tier = refreshed.rate_limit_tier;
                }
            }
        }

        let access_token = access_token.ok_or_else(|| anyhow!("missing access_token"))?;
        match channel {
            ChannelKind::ClaudeCode => {
                if user_email.is_none()
                    || organization_uuid.is_none()
                    || subscription_type.is_none()
                    || rate_limit_tier.is_none()
                {
                    let profile = fetch_oauth_profile(&access_token).await?;
                    if user_email.is_none() {
                        user_email = profile.email;
                    }
                    if organization_uuid.is_none() {
                        organization_uuid = profile.organization_uuid;
                    }
                    if subscription_type.is_none() {
                        subscription_type = profile.subscription_type;
                    }
                    if rate_limit_tier.is_none() {
                        rate_limit_tier = profile.rate_limit_tier;
                    }
                }
            }
            ChannelKind::Codex => {
                if account_id.is_none() {
                    return Err(anyhow!("missing account_id"));
                }
            }
        }

        Ok(CredentialUpsertInput {
            id: input
                .id
                .clone()
                .or_else(|| existing.as_ref().map(|item| item.id.clone())),
            channel: Some(channel),
            enabled: input
                .enabled
                .or_else(|| existing.as_ref().map(|item| item.enabled)),
            order: input
                .order
                .or_else(|| existing.as_ref().map(|item| item.order)),
            access_token: Some(access_token),
            refresh_token,
            id_token: None,
            expires_at_unix_ms: Some(expires_at_unix_ms.unwrap_or(0)),
            user_email,
            account_id,
            organization_uuid,
            subscription_type,
            rate_limit_tier,
        })
    }

    async fn build_usage_payload(
        &self,
        channel: ChannelKind,
        credentials: Vec<CredentialConfig>,
    ) -> Vec<serde_json::Value> {
        let now = now_unix_ms();
        let mut items = Vec::new();
        for credential in credentials
            .into_iter()
            .filter(|item| item.channel == channel)
        {
            let usage = match channel {
                ChannelKind::ClaudeCode => fetch_claudecode_usage(&credential.access_token).await,
                ChannelKind::Codex => match credential.account_id.as_deref() {
                    Some(account_id) if !account_id.trim().is_empty() => {
                        fetch_codex_usage(&credential.access_token, account_id).await
                    }
                    _ => Err(anyhow!("missing account_id")),
                },
            }
            .unwrap_or_else(|err| CredentialUsageSnapshot {
                last_error: Some(err.to_string()),
                ..CredentialUsageSnapshot::default()
            });
            let view = build_usage_view(&credential, usage, now);
            items.push(json!({
                "id": view.id,
                "channel": view.channel,
                "user_email": view.user_email,
                "enabled": view.enabled,
                "order": view.order,
                "status": view.status,
                "cooldown_until_unix_ms": view.cooldown_until_unix_ms,
                "last_error": view.last_error,
                "last_used_at_unix_ms": view.last_used_at_unix_ms,
                "usage": view.usage,
                "json": credential.json_view(),
            }));
        }
        items
    }
}

fn ensure_channel_credential(doc: &DurableStateDoc, channel: ChannelKind, id: &str) -> Result<()> {
    if doc
        .credentials
        .iter()
        .any(|item| item.channel == channel && item.id == id)
    {
        Ok(())
    } else {
        Err(anyhow!("credential not found: {id}"))
    }
}

fn bearer_token(headers: &Headers) -> Option<String> {
    let header = headers.get("authorization").ok().flatten()?;
    let value = header.strip_prefix("Bearer ")?;
    let value = value.trim();
    (!value.is_empty()).then(|| value.to_string())
}

fn json_error(status: u16, message: &str) -> worker::Result<Response> {
    Response::from_json(&json!({ "error": message })).map(|resp| resp.with_status(status))
}

fn request_subagent_is_review(req: &Request) -> bool {
    req.headers()
        .get("x-openai-subagent")
        .ok()
        .flatten()
        .map(|value| value.trim().eq_ignore_ascii_case("review"))
        .unwrap_or(false)
}
