# kaelix

## stack

| Layer    | Tech                              |
| -------- | --------------------------------- |
| Frontend | Next.js 16, React 19, Tailwind v4 |
| Backend  | Rust, Axum                        |
| Infra    | Docker Compose, Postgres, Redis   |

## getting started

```bash
# start the complete stack (frontend, API, postgres, redis)
cp .env.example .env
docker compose up --build

# frontend: http://localhost:3000
# API:      http://localhost:8000

# For local development without containerizing the app:
# docker compose up -d postgres redis
# cd Rust && DATABASE_URL=postgres://kaelix:kaelix@localhost:5432/kaelix cargo run
# cd JS && bun dev
```

Copy `.env.example` to `.env` and configure as needed.

## structure

```
kaelix/
├── Rust/           # Rust + Axum API
├── JS/             # Next.js App Router + Tailwind
├── docker-compose.yml
└── .env.example
```
