---
status: accepted
---

# Rust is nightly, pinned by date

`apps/api/rust-toolchain.toml` pins a dated nightly (`nightly-2026-08-22`). Gavin's other Rust projects use nightly
rustfmt options (`imports_granularity`, `group_imports`), which stable does not support. Pinning the date means local,
CI and Docker builds use the same compiler, and the Dockerfile reads the same file.

## Consequences

Bump the date deliberately, then run `cargo fmt` twice and the full checks. Nightly can break on update, so a bump is a
change in its own right. Do not pin Rust in `mise.toml`: rustup owns the toolchain through `rust-toolchain.toml`.
