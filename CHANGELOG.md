# 📦 Changelog

Todas as mudanças significativas neste projeto serão documentadas aqui.

---

## [0.3.0] - 2025-05-06
### ✨ Adicionado
- Plugin `kubernetes`: suporte à geração de arquivos de deployment e service.
- Suporte a sistema de plugins dinâmico: permite download e aplicação automática de novos plugins direto do GitHub via `--plugin`.

### ♻️ Modificado
- Ajustes na estrutura de diretórios para suportar múltiplos plugins externos.
- Refatoração do sistema de aplicação de plugins para maior flexibilidade e isolamento.

### 🔒 Infraestrutura
- Estrutura de contribuição adicionada com `.github` (Issue Templates, PR Template, Contributing).

---

## [0.2.0] - 2025-04-28
### 🚀 Início do projeto
- Implementação do CLI inicial `fastgen` com suporte a:
  - Geração de projetos a partir de blueprint YAML
  - Estrutura básica de microsserviços com Docker e FastAPI
