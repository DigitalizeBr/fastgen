# ⚡ fastgen - Cloud Native Microservices Generator with FastAPI

`fastgen` is a command-line interface (CLI) tool built in **Rust**, inspired by **Quarkus**, designed to simplify and accelerate the creation of **Cloud Native** projects using **FastAPI** in Python.

It enables developers to quickly scaffold a monorepo with multiple FastAPI microservices, ready for use with `uv`, `Docker`, `docker-compose`, and environment variables managed via `.env`.

---

## 🚀 Features

- Monorepo generation with a structured `pyproject.toml` for `uv`
- Automatic microservice creation with:
  - `main.py` base application
  - `Dockerfile` and `requirements.txt`
  - `pyproject.toml` pre-configured for `uv dev`
- Uses `uv init --no-workspace --app` in each service
- Automatic updates to `docker-compose.yml` and `.env`
- Dynamic port assignment per service
- External service catalog: PostgreSQL, Redis, RabbitMQ, MongoDB, MinIO, Keycloak
- Support for reusable plugins
- Plugins for automated testing (unit and BDD)
- Plugin for Kubernetes manifest generation
- Simple Dev UI to inspect your workspace

---

## 🛠️ Available Commands

```bash
fastgen new-workspace --name my-platform
fastgen add-service --name auth --to my-platform
fastgen add-ext --name postgresql --to my-platform
fastgen dev-ui --repo my-platform
```

> **Note on `<REPO>` / `--to` parameters:** When you see parameters like `--to <path>` or `--repo <path>`, they refer to the **local path** (directory) where your workspace was generated (e.g. `my-platform` or `./my-platform`), not a remote GitHub URL.

---

## 📦 Requirements

- [Docker](https://www.docker.com/)
- [Rust](https://www.rust-lang.org/tools/install)
- [Python ≥ 3.10](https://www.python.org/)
- [uv (from Astral)](https://github.com/astral-sh/uv)

### Compilation / Installation

To use FastGen from source, clone the repository and compile it for your platform (Linux, macOS, Windows) using the Rust `cargo` tool:

```bash
git clone https://github.com/DigitalizeBr/fastgen.git
cd fastgen

# Compile a release for your platform
cargo build --release

# The binary will be located at:
# target/release/fastgen (Linux/macOS)
# target\release\fastgen.exe (Windows)

# Optionally, install it on your system:
cargo install --path .
```

Install `uv` via:

```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

---

## 🧪 Full Example

```bash
fastgen new-workspace --name company

fastgen add-service --name users --to company
fastgen add-service --name orders --to company

fastgen add-ext --name redis --to company
fastgen add-ext --name postgresql --to company
```

Result:

```
company/
├── services/
│   ├── users/
│   │   ├── main.py
│   │   ├── Dockerfile
│   │   └── pyproject.toml
│   └── orders/
├── docker-compose.yml
├── .env
└── pyproject.toml  # with properly configured members
```

---

## ▶️ Running Locally

```bash
cd company
docker compose up
```

---

## 🔌 Available External Extensions (`add-ext`)

| Name        | Description                                | Port(s)               |
|-------------|--------------------------------------------|------------------------|
| `postgresql`| Relational database                        | 5432                   |
| `redis`     | In-memory storage                          | 6379                   |
| `rabbitmq`  | AMQP message broker                        | 5672 (AMQP), 15672 (UI)|
| `mongodb`   | Document-based NoSQL database              | 27017                  |
| `minio`     | S3-compatible object storage               | 9000                   |
| `keycloak`  | Federated authentication and authorization| 8080                   |

```bash
fastgen add-ext --name redis --to company
fastgen add-ext --name postgresql --to company
```

---

## 🧩 Reusable Plugins (`plugin`)

### 🔐 JWT Authentication Plugin

```yaml
name: auth-jwt
description: Adds JWT authentication
targets:
  - path: app/routes/auth.py
    template: auth.py
  - path: requirements.txt
    append: "
python-jose
passlib[bcrypt]"
```

```bash
fastgen plugin --name auth-jwt --project company/services/users
```

### 🧪 Unit Testing and BDD Plugin

```yaml
name: testing_bdd
description: Adds support for unit testing with pytest and BDD with pytest-bdd.
targets:
  - path: tests
    copy: true
  - path: features
    copy: true
  - path: requirements.txt
    append: |
      pytest
      pytest-bdd
```

```bash
fastgen plugin --name testing_bdd --project company/services/orders
```

### ☁️ Kubernetes Plugin

```yaml
name: kubernetes
description: Generates Kubernetes manifests (deployment + service)
targets:
  - path: k8s
    copy: true
```

```bash
fastgen plugin --name kubernetes --project company/services/users
```

---

## 📂 Plugin Structure

All plugins must be placed in:

```
templates/plugins/<plugin-name>/
```

Each must include a valid `plugin.yaml` and all necessary files/templates.

---

## ☁️ Remote Plugins

If the plugin doesn't exist locally, FastGen will fetch it from the official repository:

🔗 https://github.com/DigitalizeBr/fastgen

---

## ⚙️ Configuration via `config.yaml`

```yaml
github_token: "your_token_here"
default_author: "Your Name"

# AI Configuration for `ai-generate`
llm_provider: "ollama"           # Can be "openai", "gemini", or "ollama"
llm_model: "llama3"              # E.g., "gpt-4o", "gemini-1.5-pro", or "llama3"
openai_api_key: "sk-..."         # Required if provider is openai
gemini_api_key: "AIza..."        # Required if provider is gemini
ollama_url: "http://localhost:11434" # Required if provider is ollama
```

This file is `.gitignore`d and won't be committed.

---

## 🤖 Cloud Native Generation via AI and Dev UI

FastGen allows you to generate complete microservices and cloud-native infrastructure using modern AIs (Ollama, OpenAI, Gemini). You can monitor your workspace and leverage AI generation through a Command Line Interface (CLI) or the visual Dev UI.

### ⚙️ Configuring AI Agents

AI configuration is handled in the `config.yaml` file (usually generated in the root directory where you execute fastgen; if it doesn't exist, it will be created during operations that require it). You must choose the provider and model that best suit your needs.

Example `config.yaml`:
```yaml
github_token: "your_token_here"
default_author: "Your Name"

# AI Configuration for `ai-generate`
llm_provider: "openai"           # Can be "openai", "gemini", or "ollama"
llm_model: "gpt-4o"              # E.g., "gpt-4o", "gemini-1.5-pro", or "llama3"

# Fill in the key corresponding to your provider:
openai_api_key: "sk-yourkeyhere"
gemini_api_key: "AIza-yourkeyhere"
ollama_url: "http://localhost:11434" # If using local Ollama
```

### 🎨 How to Use the Dev UI Environment

FastGen features a Dev UI that allows you to view configured services, installed plugins, and interactively access the AI Generator.

To start the Dev UI, simply run:
```bash
# Starts the visual interface for an existing repository or an AI manifests directory
fastgen dev-ui --repo my-platform --ai-path ./my-manifests
```
Access **http://localhost:9000** in your browser.
- In the **Workspace** tab, you can view the generated services and extensions.
- In the **🤖 AI Generator** tab, you can see your AI manifests, monitor generation status, and trigger the generation process directly from the interface with a single click.

### 🚀 Tutorial: Complete AI Worklfow (Creating a project and automating services with AI)

FastGen allows you to easily initialize a workspace and automatically prepare your services for AI generation via the `--ai` flag. Let's create a complete system (e.g., a Products API and a Notifications Service) from scratch using AI with Python, `uv`, and FastAPI.

**Step 1: Initialize the Workspace**
Create your base platform.
```bash
fastgen new-workspace --name ai-platform
```

**Step 2: Add services with AI Manifests**
Use the `--ai` flag when adding a service. This will create the standard `uv` + FastAPI structure and also generate a default `instrucoes.md` template for your AI generation.
```bash
fastgen add-service --name products --to ai-platform --ai
fastgen add-service --name notifications --to ai-platform --ai
```

**Step 3: Write your instructions**
Navigate into each service folder (`ai-platform/services/products` and `ai-platform/services/notifications`) and edit the `instrucoes.md` file. Provide detailed instructions for the AI on what the FastAPI application should do.

*Example for `ai-platform/services/products/instrucoes.md`:*

```markdown
# Instructions for the Products Service
Create a FastAPI application to manage a product catalog.
- Must use Pydantic for Product models (id, name, price, description).
- Implement GET /products, POST /products, and GET /products/{id} routes.
- Use an in-memory database (a simple Python dictionary or list) for temporary data storage.
- Ensure the application runs on port 8001.
```

*Example for `ai-platform/services/notifications/instrucoes.md`:*
```markdown
# Instructions for the Notifications Service
Create a FastAPI application to send notifications.
- Include a POST /send route with a payload containing (email, message).
- Simply print the message to the console to simulate sending.
- Must run on port 8002.
```

*(Optional) Global Rules:* You can create an `ai-platform/services/validation/security_rules.md` to dictate security standards for all services:
```markdown
The code must not contain hardcoded secret keys or passwords (like database passwords in the middle of the code).
Verify that Pydantic is used in all requests that receive a data body.
```

**Step 4: AI Generation Execution**

You have two options to run your prompts and generate the Python code.

**Option A - Via Graphical Interface (Dev UI):**
```bash
fastgen dev-ui --ai-path ./ai-platform/services
```
- Go to `http://localhost:9000`
- Navigate to the **🤖 AI Generator** tab
- You will see the "products" and "notifications" manifests. Click **"Generate All"** or generate them individually. The agent will plan, write the files, and validate them. The progress will be displayed on screen.

**Option B - Via Command Line (CLI):**
```bash
fastgen ai-generate --path ./ai-platform/services
```
- The CLI will read the manifests folder.
- For each subfolder (products, notifications), it will propose an **architecture plan** and display it in the terminal.
- Press `Y` to approve.
- The AI writes the code in the same folder, and immediately after, the *Validation Agent* reviews the application to ensure it meets the rules set in `validation/`.

That's it! Once finished, your `products` and `notifications` folders will be populated with `.py` files, requirements, Dockerfiles (if instructed), and you will have a brand-new application built entirely by AI!

---

## 🤝 Contribute!

We welcome contributions!

If you enjoy **Rust**, **Python**, **software architecture**, **DevOps**, or want to help with **documentation, testing, or plugins** — join us!
