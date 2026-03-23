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

## 🤖 Geração Cloud Native via Inteligência Artificial e Dev UI

O FastGen permite que você gere microsserviços inteiros e infraestrutura nativa na nuvem usando IAs modernas (Ollama, OpenAI, Gemini). Você pode acompanhar o ambiente de trabalho e utilizar a IA através de uma interface de linha de comando (CLI) ou através do Dev UI (interface gráfica web).

### ⚙️ Configurando os Agentes de IA

A configuração da IA é feita no arquivo `config.yaml` (geralmente gerado na raiz do diretório onde você executa o fastgen, caso não exista, ele será criado em execuções que demandem configuração). Você deve escolher o provedor e o modelo que melhor te atendem.

Exemplo de `config.yaml`:
```yaml
github_token: "seu_token_aqui"
default_author: "Seu Nome"

# Configurações de Inteligência Artificial para `ai-generate`
llm_provider: "openai"           # Pode ser "openai", "gemini" ou "ollama"
llm_model: "gpt-4o"              # Ex: "gpt-4o", "gemini-1.5-pro", ou "llama3"

# Preencha a chave correspondente ao seu provider:
openai_api_key: "sk-suachaveaqui"
gemini_api_key: "AIza-suachaveaqui"
ollama_url: "http://localhost:11434" # Caso use Ollama local
```

### 🎨 Como usar o Ambiente Dev UI

O FastGen possui um Dev UI que permite visualizar os serviços configurados, plugins instalados e também acessar o gerador de IA interativamente.

Para iniciar a Dev UI, basta rodar:
```bash
# Inicia a interface visual para um repositório existente ou um diretório de manifestos de IA
fastgen dev-ui --repo minha-plataforma --ai-path ./meus-manifestos
```
Acesse **http://localhost:9000** em seu navegador.
- Na aba **Workspace**, você pode ver os serviços gerados e extensões.
- Na aba **🤖 AI Generator**, você pode visualizar os manifestos da IA, ver o status de geração e iniciar o processo de geração diretamente da interface com um clique.

### 🚀 Tutorial: Criando um Projeto Totalmente Novo com IA

Vamos criar um sistema completo (ex: uma API de Produtos e um Serviço de Notificações) do zero usando IA.

**Passo 1: Crie um diretório para seus manifestos**
Esse diretório será lido pelos Agentes de IA para entender o que deve ser codificado.
```bash
mkdir meus-manifestos
```

**Passo 2: Crie a estrutura de diretórios e escreva seus manifestos**
Dentro do diretório de manifestos, você cria uma pasta para cada microsserviço que a IA deve gerar, e coloca um arquivo Markdown (`.md`) descrevendo as regras. Você também pode incluir uma pasta especial `validation` para regras globais de revisão.

Estrutura recomendada:
```
meus-manifestos/
├── produtos/
│   └── regras.md
├── notificacoes/
│   └── instrucoes.md
└── validation/
    └── regras_seguranca.md
```

**Exemplo de Manifesto 1:** `meus-manifestos/produtos/regras.md`
```markdown
# Microsserviço de Produtos
Crie uma aplicação FastAPI para gerenciar um catálogo de produtos.
- Deve usar Pydantic para os modelos de Produto (id, nome, preco, descricao).
- Deve criar as rotas GET /produtos, POST /produtos, GET /produtos/{id}.
- Utilize um banco de dados em memória (um simples dicionário ou lista Python) para armazenar os dados de forma temporária.
- Garanta que a aplicação rode na porta 8001.
```

**Exemplo de Manifesto 2:** `meus-manifestos/notificacoes/instrucoes.md`
```markdown
# Microsserviço de Notificações
Crie uma aplicação FastAPI para envio de notificações.
- Rota POST /enviar com payload contendo (email, mensagem).
- Apenas imprima a mensagem no console, simulando o envio.
- Deve rodar na porta 8002.
```

**Exemplo de Regra de Validação:** `meus-manifestos/validation/regras_seguranca.md`
```markdown
O código não pode conter chaves secretas ou senhas hardcoded (como senhas de banco de dados no meio do código).
Verifique se Pydantic está sendo usado em todas as requisições que recebem corpo de dados.
```

**Passo 3: Geração de Código**

Você tem duas opções para gerar este projeto:

**Opção A - Pela Interface Gráfica (Dev UI):**
```bash
fastgen dev-ui --ai-path ./meus-manifestos
```
- Acesse `http://localhost:9000`
- Vá até a aba **🤖 AI Generator**
- Você verá os manifestos "produtos" e "notificacoes". Clique em **"Generate All"** ou gere individualmente. O agente vai planejar, criar os arquivos e validá-los. O progresso aparecerá na tela.

**Opção B - Pela Linha de Comando (CLI):**
```bash
fastgen ai-generate --path ./meus-manifestos
```
- A CLI vai ler a pasta de manifestos.
- Para cada pasta (produtos, notificacoes), ela irá propor um **plano de arquitetura** e exibirá no terminal.
- Você pressiona `Y` para aprovar.
- A IA escreve o código na mesma pasta, e em seguida, o *Agente de Validação* vai revisar a aplicação para garantir a qualidade de acordo com as regras em `validation/`.

Pronto! Ao fim do processo, suas pastas `produtos` e `notificacoes` estarão preenchidas com arquivos `.py`, requirements, Dockerfiles (se instruído na IA), e você terá uma aplicação nova construída inteiramente pela IA!

---

## 🤝 Contribua!

Contribuições são bem-vindas!