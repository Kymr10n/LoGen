# gui-tauri

This crate is a thin Tauri wrapper around the `logo_gen` library and the `frontend/` React app.

Development

1. Start the frontend dev server:

```bash
cd frontend
npm install
npm run dev
```

2. In a separate terminal, run the Tauri Rust app (it will load the dev server):

```bash
cd gui-tauri
cargo run
```

Production build

```bash
cd frontend
npm run build
cd ../gui-tauri
cargo build --release
# then bundle with tauri bundler if desired
```
