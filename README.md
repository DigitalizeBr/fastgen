# 🐍 fastgen - Gerador de Microsserviços Cloud Native com FastAPI

O `fastgen` é uma ferramenta CLI desenvolvida em Rust para criar rapidamente workspaces com múltiplos microsserviços Python (FastAPI) organizados em monorepo com suporte ao `uv`, `Docker`, `docker-compose` e `.env`.

## 🚀 Funcionalidades

- Criação de monorepo com `uv` + `pyproject.toml`
- Geração de microsserviços FastAPI com Dockerfile e requirements.txt
- Atualização automática do docker-compose.yml
- Geração de portas dinâmicas e variáveis no .env
- Catálogo de serviços externos: PostgreSQL, RabbitMQ, Redis, MongoDB, MinIO, Keycloak

## 🛠 Comandos

```bash
fastgen new-workspace --name minha-plataforma
fastgen add-service --name auth --to minha-plataforma
fastgen add-ext --name postgresql --to minha-plataforma
```

## 📦 Requisitos

- Rust instalado (`cargo`)
- Python ≥ 3.10
- `uv` instalado: https://github.com/astral-sh/uv

## 🧪 Exemplo

```bash
fastgen new-workspace --name empresa
fastgen add-service --name users --to empresa
fastgen add-service --name orders --to empresa
fastgen add-ext --name redis --to empresa
```

Pronto! Agora você pode rodar:

```bash
cd empresa
docker compose up
```