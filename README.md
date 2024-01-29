# Rust Shorten

A simple URL shortener written in Rust Axum.

## Features

- adding new URLs
- deleting URLs
- redirecting to the original URL

## Example environment variables

```bash
REDIS_URL=redis://localhost:6379
PORT=3000
```

## Running the app

After setting the environment variables, run the following command:

API documentation will be available at `http://localhost:3000/docs`.

```bash
cargo run
```

## Running the tests

```bash
cargo test
```
