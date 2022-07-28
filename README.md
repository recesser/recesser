# Recesser

**A Git-Based Automation System for Reproducible Computational Social Science Research Running on
Kubernetes**

1. [Installation](#installation)
2. [Usage](#usage)
3. [Development](#development)

## Installation

### CLI

You need to have `ssh-keygen` installed. This is usually part of the `openssh` package.

On Ubuntu, OpenSSH should already be installed but if it isn't you can do it yourself with `sudo`:

```bash
apt install openssh
```

The actual CLI is available on crates.io and can be installed with:

```bash
cargo install recesser-cli
```

Alternatively, [pre-compiled binaries](https://github.com/recesser/recesser/releases) are available
for Linux. Keep in mind tha the pre-compiled binaries also need ssh-keygen to be installed.

## Usage

The primary mode of interaction with the system is through Git (for the source code of your
analyses) and the CLI (to manage artifacts and administer the overall system).

### CLI

The CLI is organized in git-like subcommands. Run `rcssr <subcommand> help` to get more information
about how to use it.

```
rcssr 0.1.0

USAGE:
    rcssr [OPTIONS] <SUBCOMMAND>

OPTIONS:
        --config <CONFIG>    Path to config file
    -h, --host <HOST>        URL of system
        --help               Print help information
    -t, --token <TOKEN>      Access token
    -v, --verbose            Print verbose output
    -V, --version            Print version information

SUBCOMMANDS:
    admin         Administrate system
    artifact      Manage artifacts
    help          Print this message or the help of the given subcommand(s)
    repository    Manage repositories
```

### Backend Infrastructure

The backend infrastructure of Recesser is installed on a single Kubernetes cluster. The installation
manifests can be found in the `manifests` directory. If you configure your remote cluster, you can
leverage skaffold to deploy the entire system in one go:

```bash
skaffold run
```

## Development

The entire system can be run in a local local minikube cluster via skaffold. You need to have
`minikube` and `skaffold` installed on your machine.

First, start minikube. The system is only tested on Kubernetes `v1.24.1`.

```bash
minikube start --kubernetes-version=v1.24.1
```

Then, build and deploy all containers defined in the `manifests` directory. Skaffold will
automatically detect that your kubectl config points to minikube and deploy to it:

```bash
skaffold run
```

To run the CLI locally, you can directly compile it. You need to have the Rust toolchain installed
for this:

```bash
cargo run -p recesser-cli -- help
```

### Smoke Test

This repository contains a smoke test script that

- Removes old deployments
- Freshly deploys the entire system incl. the template executors
- Uploads a public dataset as an artifact
- Registers a custom repository
- Executes the workflow described in the repository

To run the smoke test, you need to have these dependencies installed:

- Rust toolchain
- minikube
- kubectl
- skaffold

You can run the smoke test with any repository that contains a valid workflow description
(`recesser.yaml`). However, I only ran the test with the
[`recesser/tensorflow-example`](https://github.com/recesser/tensorflow-example) repository. To
reproduce this, fork this repo and create a GitHub Personal Access Token with access to that private
fork. Provide that token as an environment variable. The name of the repository needs to be provided
in the GitHub `{owner}/{repository}` format (e.g. `recesser/tensorflow-example`).

Then run:

```bash
export GITHUB_TOKEN="<Your token>"
minikube start --kubernetes-version=v1.24.1
./smoke-test <name of your repository>
kubectl -n argo port-forward svc/argo-server 2746:2746
```

The last command forwards the port of argo workflows, the workflow execution engine. You can then
view the log output of the workflow by visiting `https://localhost:2746` and clicking on the
workflow tab. If the logs are not shown completely, click on `logs from the artifacts`.
