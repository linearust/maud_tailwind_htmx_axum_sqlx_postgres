# Project Overview

## Purpose
Template repository for minimal web applications with Rust stack.

## Tech Stack
- **Web Framework:** Axum
- **Database:** PostgreSQL + SQLx
- **Templating:** Maud
- **Frontend:** HTMX + Tailwind CSS
- **Sessions:** tower-sessions with SQLx store

## Structure
```
src/
├── handlers/      # HTTP handlers by route type
│   ├── pages/     # Full HTML pages (/{resource})
│   ├── forms/     # Form handlers (/forms/{resource})
│   └── actions/   # HTMX actions (/actions/{resource})
├── views/         # Maud templates
│   ├── pages/     # Page templates
│   ├── components/# Reusable components
│   ├── layout/    # Base layout, navigation
│   ├── helpers.rs # Display formatting (format_price, format_datetime)
│   └── response.rs# HTMX response helpers
├── data/          # Database layer
│   ├── queries/   # Read operations
│   └── commands/  # Write operations
├── models/        # Domain types
├── routes/        # Route definitions
├── middlewares/   # Tower middlewares
├── init/          # Initialization (logging, session, db)
├── auth/          # Authentication (CurrentUser, token generation)
├── email/         # Email sending (config, templates)
├── session/       # Session utilities (flash messages)
├── paths.rs       # Centralized path definitions
└── constants.rs   # Named constants (incl. validation patterns)
```
src/
├── handlers/      # HTTP handlers by route type
│   ├── pages/     # Full HTML pages (/{resource})
│   ├── forms/     # Form handlers (/forms/{resource})
│   └── actions/   # HTMX actions (/actions/{resource})
├── views/         # Maud templates
│   ├── pages/     # Page templates
│   ├── components/# Reusable components
│   └── layout/    # Base layout, navigation
├── data/          # Database layer
│   ├── queries/   # Read operations
│   └── commands/  # Write operations
├── models/        # Domain types
├── routes/        # Route definitions
├── middlewares/   # Tower middlewares
├── init/          # Initialization (logging, session, db)
├── paths.rs       # Centralized path definitions
└── constants.rs   # Named constants
```

## Conventions
- Breaking changes encouraged (no backward compatibility)
- Single standard per use case
- Everything required (no optional params)
- Fail fast with clear errors
- Constants over magic literals
