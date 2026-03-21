# SGProxy

多通道 API 凭证代理服务，基于 Cloudflare Workers + Durable Objects 构建，支持 ClaudeCode 和 Codex 双通道的凭证管理、自动轮换、OAuth 授权及用量追踪。

[![Deploy to Cloudflare](https://deploy.workers.cloudflare.com/button)](https://deploy.workers.cloudflare.com/?url=https://github.com/LeenHawk/sgproxy)

[English](./README.en.md)

## 功能特性

- **双通道支持** — ClaudeCode (Anthropic) 和 Codex (OpenAI) 双通道代理
- **智能凭证选择** — 根据速率限制和可用性自动选择最优凭证
- **OAuth 授权** — 支持完整 OAuth2 + PKCE 流程导入凭证
- **自动刷新** — Token 过期前自动刷新，刷新失败自动标记为 Dead
- **用量追踪** — 跟踪 5 小时 / 7 天窗口内的请求与 Token 用量
- **速率限制处理** — 收到 429 时自动切换到下一个可用凭证
- **管理后台** — 带深色模式和中英双语支持的 Web UI
- **公开用量页** — 无需登录即可查看凭证状态

## 技术栈

- **运行时**: Cloudflare Workers + Durable Objects (SQLite)
- **语言**: Rust → WebAssembly
- **构建**: worker-build + Cargo

## 快速开始

### 前置要求

- [Rust](https://rustup.rs/) 工具链
- [Wrangler CLI](https://developers.cloudflare.com/workers/wrangler/install-and-update/)
- Cloudflare 账号

### 本地开发

```bash
# 1. 克隆项目
git clone <repo-url> && cd sgproxy

# 2. 设置环境变量
echo 'ADMIN_TOKEN=your-secret-token' > .env

# 3. 启动开发服务器
wrangler dev
```

访问 `http://localhost:8787/` 进入管理后台。

### 部署到 Cloudflare

```bash
wrangler deploy
```

部署后需要在 Cloudflare Dashboard 中设置 `ADMIN_TOKEN` Secret。

## 使用方式

### 添加凭证

有三种方式：

1. **OAuth 导入** — 在管理后台点击 "OAuth 导入"，完成授权流程
2. **JSON 导入** — 在管理后台粘贴凭证 JSON：
   ```json
   {
     "access_token": "sk-...",
     "refresh_token": "sk-..."
   }
   ```
3. **API 导入** — 调用管理 API：
   ```bash
   curl -X POST https://your-worker.dev/api/claudecode/credentials \
     -H "Authorization: Bearer YOUR_ADMIN_TOKEN" \
     -H "Content-Type: application/json" \
     -d '{"access_token":"sk-...", "refresh_token":"sk-..."}'
   ```

### 代理请求

将客户端的 API 基地址指向你的 Worker：

- **ClaudeCode**: `https://your-worker.dev/v1/...`
- **Codex**: `https://your-worker.dev/codex/...`

代理会自动注入凭证、处理速率限制和 Token 刷新。

### 监控

- `/usage` — 公开的凭证状态与用量页面
- `/` — 管理后台（需要 ADMIN_TOKEN 登录）

## API 端点

### 代理端点

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | `/v1/*` | 代理 ClaudeCode 请求 |
| POST | `/codex/*` | 代理 Codex 请求 |

### 管理端点（需 Bearer Token 认证）

以 `/api/{channel}/` 为前缀，`{channel}` 为 `claudecode` 或 `codex`：

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/credentials` | 列出所有凭证 |
| POST | `/credentials` | 导入凭证 |
| DELETE | `/credentials/{id}` | 删除凭证 |
| POST | `/credentials/{id}/enable` | 启用凭证 |
| POST | `/credentials/{id}/disable` | 停用凭证 |
| GET | `/credentials/usage` | 查看所有凭证用量 |
| POST | `/oauth/start` | 发起 OAuth 授权 |
| POST | `/oauth/callback` | 完成 OAuth 回调 |

## 项目结构

```
src/
├── lib.rs          # 入口，路由分发到 Durable Object
├── do_state.rs     # Durable Object 实现，管理 API 路由
├── config.rs       # 数据模型、常量
├── proxy.rs        # HTTP 请求代理逻辑
├── oauth.rs        # OAuth 流程、Token 刷新、用量拉取
├── state.rs        # 存储操作、凭证选择算法
├── tokenizer.rs    # Codex Token 计数
└── web/
    └── index.html  # 单页管理后台
```
