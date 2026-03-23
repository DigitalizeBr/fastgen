# ⚡ fastgen - Gerador de Microsserviços Cloud Native com FastAPI

O `fastgen` é uma CLI desenvolvida em **Rust**, inspirada no **Quarkus**, para acelerar a criação de sistemas **Cloud Native** em **Python** com FastAPI.

Principais recursos:
- Criação de monorepo com múltiplos microsserviços
- Integração com Docker, uv e docker-compose
- Catálogo de serviços externos (Redis, PostgreSQL, etc)
- Suporte a plugins: autenticação JWT, testes TDD/BDD, Kubernetes, etc

Exemplo:
```bash
fastgen new-workspace --name empresa
fastgen add-service --name users --to empresa
fastgen plugin --name testing_bdd --project empresa/services/users
```

Contribuições são bem-vindas!
# ⚡ fastgen - Gerador de Microsserviços Cloud Native com FastAPI

O `fastgen` é uma ferramenta de linha de comando (CLI) desenvolvida em **Rust**, inspirada no **Quarkus**, com o objetivo de simplificar e acelerar a criação de projetos **Cloud Native** com **FastAPI** em Python.

Ele permite gerar rapidamente um monorepo com múltiplos microsserviços FastAPI, prontos para uso com `uv`, `Docker`, `docker-compose` e variáveis de ambiente gerenciadas via `.env`.

---

## 🚀 Funcionalidades

- Criação de monorepos com `pyproject.toml` estruturado para o `uv`
- Geração automática de microsserviços FastAPI com:
  - `main.py` com aplicação base
  - `Dockerfile` e `requirements.txt`
  - `pyproject.toml` com `[tool.uv.app]` pronto para `uv dev`
- Uso do `uv init --no-workspace --app` em cada serviço
- Atualização automática do `docker-compose.yml` e `.env`
- Geração de portas dinâmicas por serviço
- Catálogo de serviços externos: PostgreSQL, Redis, RabbitMQ, MongoDB, MinIO, Keycloak
- Suporte à extensão por plugins reutilizáveis
- Plugins para testes automatizados (unitários e BDD)
- Plugin para geração de manifests Kubernetes

---

## 🛠️ Comandos Disponíveis

```bash
fastgen new-workspace --name minha-plataforma
fastgen add-service --name auth --to minha-plataforma
fastgen add-ext --name postgresql --to minha-plataforma
```

---

## 📦 Requisitos

- [Docker](https://www.docker.com/)
- [Rust](https://www.rust-lang.org/tools/install)
- [Python ≥ 3.10](https://www.python.org/)
- [uv (da Astral)](https://github.com/astral-sh/uv)

### Compilação / Instalação

Para usar o FastGen a partir do código fonte, clone o repositório e compile-o para a sua plataforma (Linux, macOS, Windows) usando o `cargo` da linguagem Rust:

```bash
git clone https://github.com/DigitalizeBr/fastgen.git
cd fastgen

# Compilar release para sua plataforma
cargo build --release

# O binário ficará em:
# target/release/fastgen (Linux/macOS)
# target\release\fastgen.exe (Windows)

# Opcionalmente, instale-o no sistema:
cargo install --path .
```

Instale o `uv` via:

```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

---

## 🧪 Exemplo de Uso Completo

```bash
fastgen new-workspace --name empresa

fastgen add-service --name users --to empresa
fastgen add-service --name orders --to empresa

fastgen add-ext --name redis --to empresa
fastgen add-ext --name postgresql --to empresa
```

Resultado:

```
empresa/
├── services/
│   ├── users/
│   │   ├── main.py
│   │   ├── Dockerfile
│   │   └── pyproject.toml
│   └── orders/
├── docker-compose.yml
├── .env
└── pyproject.toml  # com members configurado corretamente
```

---

## ▶️ Para executar localmente:

```bash
cd empresa
docker compose up
```

---

## 🔌 Extensões Externas Disponíveis (`add-ext`)

| Nome        | Descrição                             | Porta(s)               |
|-------------|----------------------------------------|-------------------------|
| `postgresql`| Banco de dados relacional              | 5432                    |
| `redis`     | Armazenamento em memória               | 6379                    |
| `rabbitmq`  | Broker de mensageria AMQP              | 5672 (AMQP), 15672 (UI) |
| `mongodb`   | Banco de dados NoSQL baseado em documentos | 27017              |
| `minio`     | Armazenamento de objetos compatível com S3 | 9000                 |
| `keycloak`  | Autenticação e autorização federada    | 8080                    |

```bash
fastgen add-ext --name redis --to empresa
fastgen add-ext --name postgresql --to empresa
```

---

## 🧩 Plugins Reutilizáveis (`plugin`)

### 🔐 Plugin de Autenticação JWT

```yaml
name: auth-jwt
description: Adiciona autenticação JWT
targets:
  - path: app/routes/auth.py
    template: auth.py
  - path: requirements.txt
    append: "
python-jose
passlib[bcrypt]"
```

```bash
fastgen plugin --name auth-jwt --project empresa/services/users
```

### 🧪 Plugin de Testes Unitários e BDD

```yaml
name: testing_bdd
description: Adiciona suporte a testes unitários com pytest e BDD com pytest-bdd.
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
fastgen plugin --name testing_bdd --project empresa/services/orders
```

### ☁️ Plugin de Kubernetes

```yaml
name: kubernetes
description: Gera arquivos manifest (deployment + service) para Kubernetes
targets:
  - path: k8s
    copy: true
```

```bash
fastgen plugin --name kubernetes --project empresa/services/users
```

---

## 📂 Estrutura de Plugins

Todos os plugins devem estar em:

```
templates/plugins/<nome-do-plugin>/
```

---

## ☁️ Plugins Remotos

FastGen busca plugins automaticamente no GitHub, se não existirem localmente:

🔗 https://github.com/DigitalizeBr/fastgen

---

## ⚙️ Configuração via `config.yaml`

```yaml
github_token: "seu_token_aqui"
default_author: "Seu Nome"

# Configurações de Inteligência Artificial para `ai-generate`
llm_provider: "ollama"           # "openai", "gemini", ou "ollama"
llm_model: "llama3"              # Ex: "gpt-4o", "gemini-1.5-pro", ou "llama3"
openai_api_key: "sk-..."         # Requerido caso o provider seja openai
gemini_api_key: "AIza..."        # Requerido caso o provider seja gemini
ollama_url: "http://localhost:11434" # Requerido se for ollama
```

---

## 🤖 Geração Cloud Native via Inteligência Artificial

O FastGen permite que você gere microsserviços inteiros e configurações nativas na nuvem usando IAs modernas (Ollama, OpenAI, Gemini).

Para utilizá-lo, crie um diretório de manifestos. Dentro deste diretório, coloque pastas para os serviços e/ou infraestruturas desejadas. Em cada pasta, adicione um `.md` ou `.yml` detalhando o que você deseja criar. Se quiser que a IA revise o código gerado no final do processo, adicione uma pasta `validation` com os critérios de aceite.

```
meus-manifestos/
├── auth_service/
│   └── regras.md             # "Crie um serviço FastAPI focado em auth..."
├── processador_pagamentos/
│   └── fluxo.yml             # "Script python que consome RabbitMQ..."
└── validation/
    └── regras_seguranca.md   # "Sempre use Pydantic. Não use chaves hardcoded no código..."
```

**Modo Terminal (CLI):**

```bash
fastgen ai-generate --path ./meus-manifestos
```

O FastGen vai ler os arquivos, apresentar um planejamento detalhado de arquitetura na tela e, após sua aprovação (`Y/n`), irá criar os códigos de verdade diretamente na pasta. Ao final das tarefas, o *Agente de Validação* vai escanear a codebase contra suas regras.

**Modo Gráfico Web (Dev UI):**

Você também pode utilizar o modo visual no navegador:

```bash
fastgen dev-ui --repo minha-plataforma --ai-path ./meus-manifestos
```
Acesse `http://localhost:9000` para abrir a interface web (Dev UI) do FastGen, que agora possui uma aba dedicada **🤖 AI Generator**.

---

## 🤝 Contribua!

Contribuições são bem-vindas!