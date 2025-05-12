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
- [uv (da Astral)](https://github.com/astral-sh/uv):

Instalação:

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
```

---

## 🤝 Contribua!

Contribuições são bem-vindas!