
default: debug

debug:
    cargo run

devsetup:
    cp dev/hooks/* .git/hooks

fmt:
    cargo +nightly fmt --all

lint:
    cargo clippy -- -W clippy::unwrap_used -W clippy::cargo

test:
    cargo test -- --nocapture

db-up:
    docker run --rm -p 8000:8000 surrealdb/surrealdb:latest start --log debug --user root --pass root memory

repl:
    surreal sql --conn http://localhost:8000 --user root --pass root --ns test --db test

