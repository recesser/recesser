# Recesser - Reproducible Computational Social Science Research

## HTTP Server

| Method | URL                          | Description                |
| ------ | ---------------------------- | -------------------------- |
| PUT    | /artifacts                   | Upload artifact            |
| GET    | /artifacts                   | List all artifacts         |
| GET    | /artifacts/{handle}/file     | Download artifact file     |
| GET    | /artifacts/{handle}/metadata | Download artifact metadata |
| DELETE | /artifacts/{handle}          | Delete artifact            |

## Command Line Interface

| Command  | Description                        |
| -------- | ---------------------------------- |
| upload   | Upload artifact                    |
| list     | List all artifacts                 |
| download | Download artifact (incl. metadata) |
| delete   | Delete artifact                    |
