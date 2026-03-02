# aise-rust-start

Very small Rust backend.
Very simple steps.

---

## What this is

- A minimal Rust API
- Uses Axum
- Runs locally
- Deploys on Render (free)

---

## Files

```
.
├── api/
│   ├── src/
│   │   └── main.rs
│   ├── Cargo.toml
│   └── Cargo.lock
└── README.md
```

All code is in `api/`.

---

## Step 1: Check Rust

Make sure Rust is installed.

```bash
cargo --version
```

If this prints a version, you are ready.

---

## Step 2: Minimal app

File: `api/src/main.rs`

This is the smallest working app.

```rust
fn main() {
    println!("Hello, world!");
}
```

Run it:

```bash
cd api
cargo run
```

You should see:

```
Hello, world!
```

---

## Step 3: Minimal web API

Replace `api/src/main.rs` with this.

```rust
use axum::{routing::get, Router};

async fn hello() -> &'static str {
    "Hello from Rust!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/hello", get(hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
```

Run it:

```bash
cd api
cargo run
```

Open in browser:

```
http://localhost:3000/api/hello
```

---

## Step 4: Deploy (Render)

- Create a Web Service
- Language: Rust
- Root Directory: `api`
- Build Command:

```
cargo build --release
```

- Start Command:

```
cargo run --release
```

Render will give you a public URL.

---

## Status

- Backend works
- Frontend comes next
- Auth and database later

Keep it small.
One step at a time.

