ARG RUST_VERSION
ARG TARGET=x86_64-unknown-linux-gnu
ARG DISTROLESS_IMG=cc
ARG BINARY

FROM rust:${RUST_VERSION} AS chef
ARG TARGET
RUN apt-get update && \
    apt-get install --yes --quiet \
        cmake \
        musl-dev \
        musl-tools && \
    rustup target add x86_64-unknown-linux-musl && \
    cargo install cargo-chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
ARG TARGET
ARG BINARY
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --target ${TARGET} --recipe-path recipe.json
COPY . .
RUN cargo build --target ${TARGET} --bin ${BINARY}

FROM gcr.io/distroless/${DISTROLESS_IMG}
ARG TARGET
ARG BINARY
WORKDIR /usr/local/bin
COPY --from=builder /app/target/${TARGET}/debug/${BINARY} /usr/local/bin/
