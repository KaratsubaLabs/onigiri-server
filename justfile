
default: debug

debug:
    RUST_LOG=onigiri_server=debug,info cargo watch -x run

run:
    RUST_LOG=onigiri_server=debug,info cargo run

devsetup:
    cp dev/hooks/* .git/hooks

book:
    mdbook serve book

fmt:
    cargo +nightly fmt --all

lint:
    cargo clippy -- -W clippy::unwrap_used -W clippy::cargo

test:
    RUST_LOG=onigiri_server=debug cargo test -- --nocapture

db-up:
    docker run --rm -p 8000:8000 surrealdb/surrealdb:latest start --log debug --user admin --pass password memory

repl:
    surreal sql --conn http://localhost:8000 --user admin --pass password --ns onigiri --db onigiri

