FROM rust:1.58.1 AS chef 
RUN apt-get update && \
    apt-get install -yq \
    musl-dev \
    musl-tools \
    && \
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
RUN cargo build --release --bin recesser-server

FROM gcr.io/distroless/cc
WORKDIR app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/recesser-server /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/recesser-server"]
