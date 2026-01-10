# backend

Tiny HTTP backend exposing the `logo_gen` functionality for integration with frontends.

Run locally:

```bash
cd backend
cargo run
```

Example request (PNG):

```bash
curl -X POST http://localhost:3000/generate \
  -H 'Content-Type: application/json' \
  -d '{"input":"example seed","format":"png"}' --output out.png
```

Example request (SVG):

```bash
curl -X POST http://localhost:3000/generate \
  -H 'Content-Type: application/json' \
  -d '{"input":"example seed","format":"svg"}'
```
