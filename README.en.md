# SGProxy

A multi-channel API credential proxy service built on Cloudflare Workers + Durable Objects. Manages credentials for ClaudeCode and Codex with automatic rotation, OAuth, and usage tracking.

[![Deploy to Cloudflare](https://deploy.workers.cloudflare.com/button)](https://deploy.workers.cloudflare.com/?url=https://github.com/LeenHawk/sgproxy)

[中文](./README.md)

## Features

- **Dual-channel support** — Proxy for ClaudeCode (Anthropic) and Codex (OpenAI)
- **Smart credential selection** — Automatically picks the best credential based on rate limits
- **OAuth integration** — Full OAuth2 + PKCE flow for credential import
- **Auto-refresh** — Tokens refreshed before expiration; failed refreshes mark credentials as Dead
- **Usage tracking** — Monitors request and token usage across 5-hour / 7-day windows
- **Rate limit handling** — Automatically rotates to next credential on 429 responses
- **Admin UI** — Web dashboard with dark mode and i18n (Chinese / English)
- **Public usage page** — View credential status without authentication

## Tech Stack

- **Runtime**: Cloudflare Workers + Durable Objects (SQLite)
- **Language**: Rust → WebAssembly
- **Build**: worker-build + Cargo

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) toolchain
- [Wrangler CLI](https://developers.cloudflare.com/workers/wrangler/install-and-update/)
- Cloudflare account

### Local Development

```bash
# 1. Clone the repo
git clone <repo-url> && cd sgproxy

# 2. Set environment variables
echo 'ADMIN_TOKEN=your-secret-token' > .env

# 3. Start the dev server
wrangler dev
```

Open `http://localhost:8787/` to access the admin dashboard.

### Deploy to Cloudflare

```bash
wrangler deploy
```

After deploying, set the `ADMIN_TOKEN` secret in Cloudflare Dashboard.

## Usage

### Adding Credentials

Three methods:

1. **OAuth Import** — Click "OAuth Import" in the admin UI and complete the authorization flow
2. **JSON Import** — Paste credential JSON in the admin UI:
   ```json
   {
     "access_token": "sk-...",
     "refresh_token": "sk-..."
   }
   ```
3. **API Import** — Call the management API:
   ```bash
   curl -X POST https://your-worker.dev/api/claudecode/credentials \
     -H "Authorization: Bearer YOUR_ADMIN_TOKEN" \
     -H "Content-Type: application/json" \
     -d '{"access_token":"sk-...", "refresh_token":"sk-..."}'
   ```

### Proxying Requests

Point your client's API base URL to your Worker:

- **ClaudeCode**: `https://your-worker.dev/v1/...`
- **Codex**: `https://your-worker.dev/codex/...`

The proxy automatically injects credentials, handles rate limits, and refreshes tokens.

### Monitoring

- `/usage` — Public credential status and usage page
- `/` — Admin dashboard (requires ADMIN_TOKEN login)

## API Endpoints

### Proxy Endpoints

| Method | Path | Description |
|--------|------|-------------|
| POST | `/v1/*` | Proxy ClaudeCode requests |
| POST | `/codex/*` | Proxy Codex requests |

### Management Endpoints (Bearer Token required)

Prefixed with `/api/{channel}/`, where `{channel}` is `claudecode` or `codex`:

| Method | Path | Description |
|--------|------|-------------|
| GET | `/credentials` | List all credentials |
| POST | `/credentials` | Import a credential |
| DELETE | `/credentials/{id}` | Delete a credential |
| POST | `/credentials/{id}/enable` | Enable a credential |
| POST | `/credentials/{id}/disable` | Disable a credential |
| GET | `/credentials/usage` | View usage for all credentials |
| POST | `/oauth/start` | Start OAuth flow |
| POST | `/oauth/callback` | Complete OAuth callback |

## Project Structure

```
src/
├── lib.rs          # Entry point, routes requests to Durable Object
├── do_state.rs     # Durable Object implementation, management API routes
├── config.rs       # Data models, constants
├── proxy.rs        # HTTP request proxying logic
├── oauth.rs        # OAuth flows, token refresh, usage fetching
├── state.rs        # Storage operations, credential selection algorithm
├── tokenizer.rs    # Codex token counting
└── web/
    └── index.html  # Single-page admin dashboard
```
