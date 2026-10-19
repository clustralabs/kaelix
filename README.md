# kaelix

## stack

| Layer    | Tech                              |
| -------- | --------------------------------- |
| Frontend | Next.js 16, React 19, Tailwind v4 |
| Backend  | Rust, Axum                        |
| Database | Turso (libSQL)                    |

## getting started

Run both the frontend and backend with a single command:

```bash
./dev.sh
```

Or run them individually:

```bash
# backend (http://localhost:8000)
cd Rust && cargo run

# frontend (http://localhost:3000)
cd JS && bun dev
```

Configure `.env` as needed (defaults to local `kaelix.db` for Turso).

## structure

```
kaelix/
├── Rust/           # Rust + Axum API (Turso database integration)
├── JS/             # Next.js App Router + Tailwind
├── dev.sh          # One-click startup script for frontend and backend
├── DOCUMENTATION.md # Comprehensive architecture & technical documentation
└── .env.example
```

For detailed system architecture, API endpoints, schema, and development details, see [DOCUMENTATION.md](file:///Users/param/kaelix/DOCUMENTATION.md).

