# kaelix

## stack

| Layer    | Tech                              |
| -------- | --------------------------------- |
| Frontend | Next.js 16, React 19, Tailwind v4 |
| Backend  | Rust, Axum                        |
| Infra    | Docker Compose, Postgres, Redis   |

## getting started

```bash
# start services (postgres, redis)
docker compose up -d

# run the backend
cd Rust && cargo run

# run the frontend
cd JS && bun dev
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
