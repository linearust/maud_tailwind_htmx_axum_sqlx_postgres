# Suggested Commands

## Development
- `just run` - Pull, update submodules, run with cargo watch (RUST_LOG=debug)
- `just check` - Run clippy (with -D warnings) and tests
- `just migrate` - Run database migrations

## Setup
- `just setup` - Install sqlx-cli and cargo-watch

## Individual Commands
- `cargo clippy -- -D warnings` - Lint check
- `cargo test` - Run tests
- `sqlx migrate run` - Run migrations
