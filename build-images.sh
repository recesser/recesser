#!/bin/sh

binaries="
    apiserver
    rcssr
    schandler
"

for b in ${binaries}; do
    echo "Building ${b}..."
    docker build . --build-arg RUST_VERSION="${RUST_VERSION}" --build-arg BINARY="${b}"
done
