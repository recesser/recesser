#!/usr/bin/env bash

set -euo pipefail

requirements=$1
entrypoint=$2

pushd /src

# Install dependencies during runtime
python3 -m pip install --use-feature=2020-resolver -r "${requirements}"

# Execute entrypoint, passing it all arguments after the entrypoint arg itself
python3 "${entrypoint}" "${@:3}"
