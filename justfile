
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

repl:
    surreal sql --conn http://localhost:8000 --user root --pass root --ns test --db test

