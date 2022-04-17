#!/bin/sh

binaries="
    apiserver
    rcssr
    schandler
"

build() {
    binary=$1
    docker build . \
        --build-arg RUST_VERSION="${RUST_VERSION}" \
        --build-arg BINARY="${binary}" \
        --tag "recesser/${binary}:latest"
}

for b in ${binaries}; do
    echo "Building ${b}..."
    build ${b}
done
