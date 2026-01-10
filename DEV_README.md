# Development using Docker

This project includes a `docker-compose.yml` and Dockerfiles to run the frontend (Vite) and backend (Rust) together for local development.


Quick start:

```bash
# Build and start both services (uses the Docker Compose V2 plugin)
make up

# Open the frontend at http://localhost:5173
# The backend is available at http://localhost:3000
```

If your Docker installation provides the old `docker-compose` binary instead of the `docker compose` plugin, run:

```bash
docker-compose up --build
```

Notes:
- Frontend files are mounted into the container, so local edits are visible to Vite.
- Backend mounts the workspace into the container and runs `cargo run` by default.
