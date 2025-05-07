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

O comando `add-ext` adiciona automaticamente serviços populares ao `docker-compose.yml`, com configuração mínima e suporte a variáveis no `.env`.

Atualmente, o catálogo de extensões inclui:

| Nome        | Descrição                             | Porta(s)     |
|-------------|----------------------------------------|--------------|
| `postgresql`| Banco de dados relacional              | 5432         |
| `redis`     | Armazenamento em memória               | 6379         |
| `rabbitmq`  | Broker de mensageria AMQP              | 5672 (AMQP), 15672 (UI) |
| `mongodb`   | Banco de dados NoSQL baseado em documentos | 27017     |
| `minio`     | Armazenamento de objetos compatível com S3 | 9000      |
| `keycloak`  | Autenticação e autorização federada    | 8080         |

### 🧪 Exemplo de uso:

```bash
fastgen add-ext --name redis --to empresa
fastgen add-ext --name postgresql --to empresa
```

---

## 🧩 Criando e Usando Plugins Reutilizáveis (`plugin`)

Plugins permitem adicionar funcionalidades específicas (como autenticação, middlewares ou configurações extras) de forma reaproveitável em qualquer microsserviço.

### 📁 Estrutura de um plugin:

```
templates/
└── plugins/
    └── auth-jwt/
        ├── plugin.yaml
        └── auth.py
```

### ✍️ Exemplo de `plugin.yaml`:

```yaml
name: auth-jwt
description: Adiciona autenticação JWT
targets:
  - path: app/routes/auth.py
    template: auth.py
  - path: requirements.txt
    append: "\npython-jose\npasslib[bcrypt]"
```

### ✅ Aplicando o plugin:

```bash
fastgen plugin --name auth-jwt --project empresa/services/users
```

Isso irá:

- Renderizar o arquivo `auth.py` a partir do template
- Adicionar as dependências no `requirements.txt`

---

## 📂 Onde colocar seus plugins

Todos os plugins devem ser adicionados em:

```
templates/plugins/<nome-do-plugin>/
```

Cada plugin deve conter pelo menos um arquivo `plugin.yaml` e os arquivos/templates necessários (.py, .env, .toml etc).

---
## ☁️ Plugins Remotos via GitHub
Se o plugin não existir localmente, o fastgen o baixa do repositório oficial no GitHub:
https://github.com/DigitalizeBr/fastgen

---
## ⚙️ Configuração via config.yaml
Para configurar chaves e opções adicionais como o token do GitHub (usado para baixar plugins automaticamente), crie um arquivo config.yaml na raiz do projeto:

```bash
github_token: "seu_token_aqui"
```

Nota: este arquivo é ignorado via .gitignore e não será versionado.


---

**Contribuições são bem-vindas!**
