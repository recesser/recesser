FROM rust:1.59.0 AS chef
RUN apt-get update && \
    apt-get install --yes --quiet \
        musl-dev \
        musl-tools && \
    rustup target add x86_64-unknown-linux-musl && \
    cargo install cargo-chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin recesser-apiserver

FROM gcr.io/distroless/static
WORKDIR app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/recesser-apiserver /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/recesser-apiserver"]
