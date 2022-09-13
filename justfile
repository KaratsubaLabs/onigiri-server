
default: debug

debug:
    cargo run

devsetup:
    cp dev/hooks/* .git/hooks

fmt:
    cargo +nightly fmt --all

test:
    cargo test -- --nocapture

lint:
    cargo clippy -- -W clippy::unwrap_used -W clippy::cargo
