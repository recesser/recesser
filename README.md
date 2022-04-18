# Recesser

**A Git-Based Automation System for Reproducible Computational Social Science Research**

## Usage

The primary mode of interaction with the system is through Git (for the source code)
and the CLI (to manage artifacts and administer the overall system).

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

## Installation

### CLI

```sh
cargo install recesser-cli
```

Alternatively, [precompiled binaries](https://github.com/recesser/recesser/releases) are available
for linux.

### Backend Infrastructure

The backend infrastructure of Recesser is installed on a single Kubernetes cluster. The installation
manifests and instructions can be found in the [Recesser
Infrastructure](https://github.com/recesser/infrastructure) repository.

## Development

Start backend services with Docker Compose:

```bash
docker compose up --detach
```

Source environment variables for local development:

```bash
set -a # Necessary to export all created variables when sourcing a file
source apiserver.local.env
source cli.local.env
source schandler.local.env
```