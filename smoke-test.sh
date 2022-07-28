#!/usr/bin/env bash

set -euo pipefail

# The GitHub repository that will be deployed to recesser. You also need
# to provide a GitHub personal access token that can add deploy keys to the 
# the repostiory as the environemnt variable GITHUB_TOKEN
repository=$1

# Variables
host="http:/localhost:8080"
input_data="/tmp/aclImdb_v1.tar.gz"

# Environment variables
export RUST_VERSION=1.62

get_token() {
    kubectl get \
        --namespace recesser \
        secrets/apiserver-token \
        --template="{{.data.token}}" | 
        base64 --decode
}

upload_data() {
    cargo run --quiet --package recesser-cli -- \
        --host "${host}" \
        --token "${token}" \
        artifact \
        upload \
        "${input_data}"
}

register_repo() {
    token=$1

    cargo run --quiet --package recesser-cli -- \
        --host "${host}" \
        --token "${token}" \
        repository \
        add \
        "${repository}"
}

# Remove old resources because deployment expects fresh cluster
skaffold delete
echo "Waiting for 5 seconds for resources to be deleted"
sleep 5 # Wait for resources to be delete

# Build template-executors
eval $(minikube docker-env)
pushd template-executors/python-tensorflow
make VERSION=1.0.0
popd

# Deploy
skaffold run
echo "Waiting for 40 seconds for deployment to stabilize"
sleep 40 # Wait for deployment to stabilize

# Get initial Receesser apiserver token
token=$(get_token)
echo "[Initial Token] ${token}"

# Forward port of apiserver to localhost:8080 in the background
kubectl port-forward service/apiserver 8080:80 -n recesser > /dev/null 2>&1 &
sleep 1 # Sleep for a while otherwise the next command fails
trap 'kill %kubectl' EXIT # Kill port forward once script exits

# Download data for tensorflow-example if it doesn't exist yet
if [[ ! -f "${input_data}" ]]; then
    wget http://ai.stanford.edu/~amaas/data/sentiment/aclImdb_v1.tar.gz -O "${input_data}"
fi
# Check integrity of data
checksum="c40f74a18d3b61f90feba1e17730e0d38e8b97c05fde7008942e91923d1658fe  ${input_data}"
echo "${checksum}" | sha256sum --check --status

# Upload data
artifact_handle=$(upload_data)
echo "[Artifact Handle] ${artifact_handle}"

# Register repository and get SSH key
ssh_key=$(register_repo "${token}")
echo "[SSH Key] ${ssh_key}"

# Register SSH key as a deploy key in repository
echo -n "[GITHUB] "
curl \
    --silent \
    --show-error \
    "https://api.github.com/repos/${repository}/keys" \
    --header "Accept: application/vnd.github+json" \
    --header "Authorization: token ${GITHUB_TOKEN}" \
    --data "{\"title\": \"RecesserMachineKey\",\"key\":\"${ssh_key}\",\"read_only\":true}"
