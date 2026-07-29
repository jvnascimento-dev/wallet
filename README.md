# Wallet

Esse é um projeto com base do último desafio do bootcamp [Santander da DIO](https://github.com/digitalinnovationone/rust-fullstack-carteira-investimentos)
Carteira de ativos construída em **Rust** com **Axum**. 
Permite cadastrar ativos (admin), registrar compras de ativos (usuário autenticado) e acompanhar, no navegador, o histórico de compras e a variação de preço de cada ativo.

---

## Stack

| Camada            | Tecnologia                                                   |
| ----------------- | ------------------------------------------------------------ |
| Linguagem         | Rust (edition 2024)                                          |
| Web framework     | Axum 0.8                                                     |
| Banco de dados    | PostgreSQL 18 (Docker)                                       |
| ORM/Queries       | SQLx 0.9 (macros compile-time)                              |
| Templates HTML    | Askama 0.16                                                  |
| Auth              | JWT (jwt-simple) + cookies assinados (axum-extra)           |
| Hash de senha     | password-auth                                               |
| CSS               | TailwindCSS via CDN                                          |
| Observabilidade   | tracing + tracing-subscriber + color-eyre                   |
| Testes            | sqlx::test (DB efêmero) + insta (snapshots JSON)            |

---

## Arquitetura

```mermaid
flowchart LR
  Browser[Browser / Cliente]

  subgraph App["Aplicação Axum :3000"]
    direction TB
    State["AppState<br/>PgPool (Postgres)"]
    Router["Router"]

    subgraph Frontend["Rotas Frontend /"]
      FHandler["Handlers<br/>login · logout · assets<br/>purchase_asset"]
    end

    subgraph API["Rotas API /api"]
      AHandler["Handlers<br/>list · create · update assets"]
      AdminGuard["Admin Extractor<br/>Authorization header"]
    end

    subgraph Auth["Extractors de Auth"]
      UserGuard["User Extractor<br/>Cookie JWT"]
    end

    Repo["Repository<br/>SQLx queries"]

    Router --> Frontend
    Router --> API
    Frontend --> UserGuard
    API --> AdminGuard
    FHandler --> Repo
    AHandler --> Repo
    State -. compartilha PgPool .-> Repo
  end

  Browser -- HTTP --> Router
  Repo --> DB[(PostgreSQL)]
```

---

## Modelo de dados

```mermaid
erDiagram
  assets {
    BIGSERIAL id PK
    TEXT      name UK
    DOUBLE    unit_value
  }
  users {
    BIGSERIAL id PK
    TEXT      username UK
    TEXT      password_hash
  }
  owned_assets {
    BIGSERIAL id PK
    BIGINT    user_id FK
    BIGINT    asset_id FK
    DOUBLE    bought_for
    DOUBLE    quantify_owned
    TIMESTAMPTZ timestamp
  }

  users ||--o{ owned_assets : "possui"
  assets ||--o{ owned_assets : "é comprado em"
```

---

## Fluxo de autenticação

```mermaid
sequenceDiagram
  autonumber
  actor U as Usuário
  participant B as Backend Axum
  participant DB as PostgreSQL

  U->>B: POST /login (username, password)
  B->>DB: SELECT user por username
  alt usuário existe
    B->>B: verifica hash da senha
  else usuário não existe
    B->>DB: INSERT novo usuário (hash da senha)
  end
  B->>B: gera JWT (expira em 10 min)
  B-->>U: 302 Redirect para /assets<br/>Set-Cookie: token=<JWT> HttpOnly

  U->>B: GET /assets (Cookie: token=<JWT>)
  B->>B: User extractor valida JWT do cookie
  B->>DB: lista owned_assets + assets disponíveis
  B-->>U: 200 HTML renderizado (Askama)

  U->>B: GET /logout
  B-->>U: 302 Redirect para /login<br/>Remove cookie token
```

---

## Estados do usuário

```mermaid
stateDiagram-v2
  [*] --> Anônimo
  Anônimo --> Autenticado : POST /login<br/>(credenciais válidas ou registro)
  Autenticado --> Autenticado : Requisições com cookie JWT válido
  Autenticado --> Anônimo : GET /logout
  Autenticado --> Anônimo : JWT expira (10 min)
  Anônimo --> [*]
```

---

## Rotas

| Método | Path        | Descrição                              | Auth                          |
| ------ | ----------- | -------------------------------------- | ----------------------------- |
| GET    | `/`         | Redireciona para `/login` ou `/assets` | Público                       |
| GET    | `/login`    | Página de login (HTML)                 | Público                       |
| POST   | `/login`    | Autentica/registra e cria cookie JWT   | Público                       |
| GET    | `/logout`   | Remove cookie e redireciona            | Público                       |
| GET    | `/assets`   | Página da carteira (HTML)              | `User` (cookie JWT)           |
| POST   | `/assets`   | Registra compra de um ativo           | `User` (cookie JWT)           |
| GET    | `/api/assets`  | Lista ativos disponíveis            | Público                       |
| POST   | `/api/assets`  | Cria novo ativo                     | `Admin` (header Authorization) |
| PATCH  | `/api/assets`  | Atualiza nome/valor de um ativo     | `Admin` (header Authorization) |

---

## Como executar

### Pré-requisitos

- [Rust toolchain](https://rustup.rs/) (estável, edition 2024)
- [Docker](https://www.docker.com/) (para o PostgreSQL)
- [`sqlx-cli`](https://github.com/launchbadge/sqlx#cli) instalado (`cargo install sqlx-cli`)

### Passos

1. Suba o banco de dados:

   ```bash
   docker compose up -d
   ```

2. Configure o arquivo `.env` na raíz com a URL do banco
   (o `.env` está no `.gitignore` — não é versionado):

   ```dotenv
   DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres
   ```

3. Aplique as migrations (a aplicação **não** roda migrations no startup;
   é necessário aplicá-las manualmente):

   ```bash
   sqlx migrate run --database-url "postgres://postgres:postgres@localhost:5432/postgres"
   ```

4. Compile e execute a aplicação:

   ```bash
   cargo run
   ```

5. Acesse no navegador: <http://localhost:3000>
   - Em `/login`, informe `username` e `password`.
     Se o usuário não existir, ele é registrado automaticamente e autenticado.

---

## Testes

Os testes validam os handlers da API com um banco PostgreSQL efêmero
(`sqlx::test`) e usam [insta](https://insta.rs/) para snapshots JSON.

```bash
cargo test
```

Os snapshots ficam em `src/routes/snapshots/` e os fixtures de teste
em `src/routes/fixtures/`.

---

## Estrutura de pastas

```text
wallet/
├── Cargo.toml
├── docker-compose.yml          # PostgreSQL 18
├── .env                        # DATABASE_URL (não versionado)
├── migrations/
│   ├── *_create_assets.up.sql
│   ├── *_create_users.up.sql
│   └── *_create_owned_assets.up.sql
├── templates/
│   ├── login.html              # Página de login (Tailwind)
│   └── assets.html             # Carteira + modal de compra
└── src/
    ├── main.rs                 # Entry point (tokio)
    ├── app.rs                  # App::start, AppState (PgPool), router
    ├── error.rs                # AppErr (thiserror) → IntoResponse
    ├── models.rs               # Asset, DtoAsset, UserRecord, OwnedAsset...
    ├── repository.rs           # Repository (SQLx) + FromRequestParts
    ├── auth/
    │   ├── mod.rs
    │   ├── user.rs             # User/UnauthUser: JWT, cookies, registro
    │   └── admin.rs           # Admin: header Authorization
    └── routes/
        ├── mod.rs
        ├── api.rs              # Rotas /api/assets + testes
        ├── frontend.rs         # Rotas /, /login, /logout, /assets
        ├── fixtures/
        └── snapshots/
```