# Recesser

**Reproducible Computational Social Science Research**

## Usage

The primary mode of interaction with the system is through the CLI.

### CLI

## Installation

Recesser is installed on a single Kubernetes cluster. The installation manifests can be found in the
[Recesser Infrastructure](https://github.com/recesser/infrastructure) repository.

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