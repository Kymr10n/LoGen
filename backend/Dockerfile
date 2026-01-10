FROM rust:latest as builder

WORKDIR /workspace

# Copy entire repo so Cargo can resolve path dependencies
COPY . .

WORKDIR /workspace/backend

# Build the backend
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /workspace/backend/target/release/backend /usr/local/bin/backend

EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/backend"]
