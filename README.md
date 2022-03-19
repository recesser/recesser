# Recesser - Reproducible Computational Social Science Research

## Apiserver

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
| GET    | /repositories/{id}              | Show repository                 |
| GET    | /repositories/{id}/credentials  | Retrieve repository credentials |
| DELETE | /repositories/{id}              | Deregister repository           |

### Users

| Method | URL                          | Description                |
| ------ | ---------------------------- | -------------------------- |
| PUT    | /users                       | Create new user            |
| GET    | /users                       | List all users             |
| DELETE | /users/{id}                  | Delete user                |

## Command Line Interface

### Repository

| Command  | Description                        |
| -------- | ---------------------------------- |
| upload   | Upload artifact                    |
| list     | List all artifacts                 |
| download | Download artifact (incl. metadata) |
| delete   | Delete artifact                    |
