#!/usr/bin/env bash

set -euo pipefail

# Directory has to point at tensorflow-example
directory=$1
input_data="aclImdb_v1.tar.gz"

# Download data for tensorflow-example if it doesn't exist yet
if [[ ! -f "${input_data}" ]]; then
    wget http://ai.stanford.edu/~amaas/data/sentiment/aclImdb_v1.tar.gz -O "${input_data}"
fi

# Check integrity of data
checksum="c40f74a18d3b61f90feba1e17730e0d38e8b97c05fde7008942e91923d1658fe  aclImdb_v1.tar.gz"
echo "${checksum}" | sha256sum --check --status

exec docker run \
    --volume "$(readlink -f ${directory}):/src" \
    --volume "$(readlink -f ${input_data}):/inputs/${input_data}" \
    recesser/python-tensorflow-executor \
    requirements.txt \
    main.py \
    "/inputs/${input_data}"
