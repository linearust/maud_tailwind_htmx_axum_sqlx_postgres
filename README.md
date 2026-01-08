# Web App Template

Rust web application template with auth, payments, sessions, and database.

**Stack:** Axum • PostgreSQL + SQLx • Maud • HTMX + Tailwind CSS

## Quick Start

```bash
cp .env.example .env  # Configure environment
just migrate          # Run migrations
just run              # Start dev server
```

See `.env.example` for required configuration.

## Structure

Routes organized by response type: Pages (`/`), Forms (`/forms/`), Actions (`/actions/`).

See `CLAUDE.md` for development guidelines.
