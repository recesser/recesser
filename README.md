# Recesser

**A Git-Based Automation System for Reproducible Computational Social Science Research**

## Installation

### CLI

You need to have `ssh-keygen` installed. This is usually part of the `openssh` package.

On Ubuntu OpenSSH should already be installed but if it isn't you can do it yourself with `sudo`:

```
apt install openssh
```

The actual CLI is available on crates.io and can be installed with:

```sh
cargo install recesser-cli
```

Alternatively, [precompiled binaries](https://github.com/recesser/recesser/releases) are available
for Linux. Keep in mind tha the precompiled binaries also need ssh-keygen to be installed.

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

```
skaffold run
```

## Development

There are three levels of local development: (1) running single components via cargo, (2) running
all non-kubernetes-native components via docker-compose, and (3) running all components in a local
minikube environment via skaffold.

### Single Component

You can source the environment variables for each component:

```bash
set -a # Necessary to export all created variables when sourcing a file
source apiserver.local.env
source cli.local.env
source schandler.local.env
```

Then run a single component in a shell:

```
cargo run -p recesser-apiserver
```

### Non-Kubernetes Components

Start the services with Docker Compose and detach so you can still use this shell:

```bash
docker compose up --detach
```

### All Components in Local Cluster

Set up minikube:

```bash
minikube start
skaffold config set local-cluster true
eval $(minikube docker-env)
```

Build and deploy all containers

```bash
skaffold run
```
