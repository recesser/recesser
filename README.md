# Recesser - Reproducible Computational Social Science Research

## HTTP Server

### Artifacts

| Method | URL                          | Description                |
| ------ | ---------------------------- | -------------------------- |
| PUT    | /artifacts                   | Upload artifact            |
| GET    | /artifacts                   | List all artifacts         |
| GET    | /artifacts/{handle}/file     | Download artifact file     |
| GET    | /artifacts/{handle}/metadata | Download artifact metadata |
| DELETE | /artifacts/{handle}          | Delete artifact            |

### Repositories

| Method | URL                             | Description                     |
| ------ | ------------------------------- | ------------------------------- |
| PUT    | /repositories                   | Register repository             |
| GET    | /repositories                   | List all repositories           |
| GET    | /repositories/{id}/credentials  | Retrieve repository credentials |
| DELETE | /repositories/{id}              | Deregister repository           |

## Command Line Interface

| Command  | Description                        |
| -------- | ---------------------------------- |
| upload   | Upload artifact                    |
| list     | List all artifacts                 |
| download | Download artifact (incl. metadata) |
| delete   | Delete artifact                    |
