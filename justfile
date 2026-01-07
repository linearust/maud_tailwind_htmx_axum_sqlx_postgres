default: run

run:
    git pull --rebase --autostash
    git submodule update --init --remote
    RUST_LOG=debug cargo watch -c -x run

check:
    cargo clippy -- -D warnings
    cargo test

setup:
    cargo install sqlx-cli --no-default-features --features postgres
    cargo install cargo-watch

migrate:
    sqlx migrate run
