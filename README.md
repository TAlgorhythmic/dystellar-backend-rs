# 🛠️ Minecraft Server Auth Backend (Minecraft login with Microsoft OAuth2)

This is a lightweight, low level, high-performance and heavily multithreaded backend written in Rust for Dystellar Network.

---

# Functional, but still in development!

---

## 🚀 Features

- 🔐 Microsoft OAuth 2.0 Login Integration
- ⚙️  Environment-based Configuration (Test & Prod)  
- ⚡ Fast and Safe – Built with Rust
- 🌐 Designed for seamless Minecraft server integration  

---

## 📦 Requirements

### Microsoft Azure App Registration

This project requires a **Microsoft Azure application** with:

- `Client ID`  
- `Client Secret`  
- `Redirect URI`  

You can register one at [https://portal.azure.com](https://portal.azure.com).

---

## 🧪 Local Development Setup

Before running the project, create a `.env` file in the root of the repository with the following entries:

```env
# Development/Test Configuration
TEST_HOST=127.0.0.1
TEST_PORT=3000
TEST_CLIENT_ID=<your microsoft client id here>
TEST_REDIRECT_URI="http://localhost:3000/api/microsoft/callback" # # Make sure that your Azure app registration redirect uri matches with this
TEST_CLIENT_SECRET=<your microsoft client secret here>
TEST_PRIVILEGED_AUTHORIZED_IP="127.0.0.1"

# Production Configuration
PROD_HOST=0.0.0.0
PROD_PORT=443
PROD_CLIENT_ID=<your microsoft client id here>
PROD_REDIRECT_URI="http://0.0.0.0:443/api/microsoft/callback" # Make sure that your Azure app registration redirect uri matches with this
PROD_CLIENT_SECRET=<your microsoft client secret here>
PROD_PRIVILEGED_AUTHORIZED_IP="0.0.0.0"

# Privileged Access Token
PRIVILEGE_TOKEN="secret token"
```

---

## 🔧 Building & Running

Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.

### Development

```bash
cargo run
```

### Production

```bash
cargo build --release
```

---

## 🧠 Notes

- Ensure the **Redirect URIs** in your Azure app registration match those in your `.env`.
- Keep `PRIVILEGE_TOKEN` secret — it's used to authorize sensitive IP-restricted routes.

