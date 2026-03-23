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

---

## 📦 Requirements

- [Docker](https://www.docker.com/)
- [Rust](https://www.rust-lang.org/tools/install)
- [Python ≥ 3.10](https://www.python.org/)
- [uv (from Astral)](https://github.com/astral-sh/uv)

Install with:

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

## 🤖 AI Generation

FastGen allows you to generate complete services and cloud-native infrastructure using AI (Ollama, OpenAI, Gemini).

To use it, create a directory with subdirectories for your services and/or infrastructure. In each subdirectory, place a `.md` or `.yml` manifest explaining what needs to be created. You can optionally include a `validation` folder with instructions on how to evaluate the final code.

```bash
fastgen ai-generate --path ./my-manifests
```

The tool will read the manifests, propose a plan for each service, ask for your approval, and generate the files directly in that folder using the configured LLM. If a validation step is defined, it will review the created files against your instructions.

---

## 🤝 Contribute!

We welcome contributions!

If you enjoy **Rust**, **Python**, **software architecture**, **DevOps**, or want to help with **documentation, testing, or plugins** — join us!
