FROM rust:slim-buster

RUN USER=root cargo new --bin onigiri_server
WORKDIR onigiri_server

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./Rocket.toml ./Rocket.toml

# cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# build the project
COPY ./src ./src
RUN rm ./target/release/deps/onigiri_server*
RUN cargo install --path .

CMD ["onigiri_server"]
