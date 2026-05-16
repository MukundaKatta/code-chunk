# code-chunk

[![crates.io](https://img.shields.io/crates/v/code-chunk.svg)](https://crates.io/crates/code-chunk)

Brace-aware code chunker for RAG. Splits source on function/class
boundaries via brace-depth tracking — no language parser needed.

```rust
use code_chunk::chunk;
let src = "fn a() { 1 }\nfn b() { 2 }";
let chunks = chunk(src, 200);
```

Soft cap: oversize functions stay whole rather than splitting mid-body.
Zero deps. MIT or Apache-2.0.

## Repository Health

This repository includes a dependency-free health check for core documentation, metadata, and CI wiring. Run it locally before publishing changes:

```sh
python3 scripts/check_repository_health.py
```

The same check runs in GitHub Actions on pushes and pull requests.
