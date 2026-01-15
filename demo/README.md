# Yap Demo

For convenience, you can test having at least 2 clients talk to each other without build or setup headaches.

This uses "Docker Compose" (calls `podman-compose`) to set up 3 containers:
1. Database using `mariadb`
2. Server using the official Debian 13-based Rust image
3. Client using the same image

## Prerequisites

- POSIX shell
- `podman`
- `podman-compose`

## Running

You'll need at least 4 ttys for this.

1. Term 1: `./run.sh prepare`
    - Creates tempdir `deploy/`
    - Copies example server config to it
    - Builds containers from `compose.yaml` (this will take a while)
2. Term 2: `./run.sh db` starts the database with Ctrl-C available
3. Wait until database is up
4. Term 3: `./run.sh server` starts the server
5. Term 1: `./run.sh register uid password`
    - `uid` should be a number
    - Configs go into `deploy/`
    - Example:
```
./run.sh register 1 passwordone
./run.sh register 2 passwordtwo
```
6. Term 1: `./run.sh client uid`
    - `./run.sh client 1` for first client and so on

To quit client, use command `/q` as signals aren't caught.
