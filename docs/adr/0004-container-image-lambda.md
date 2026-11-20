---
status: accepted
---

# The Lambda deploys as a Docker container image

The Rust API is built by Docker into an ARM64 container image, based on the official `provided.al2023` Lambda image,
and CDK builds and pushes it on `cdk deploy`. A deploy needs Docker and nothing else: no Rust cross-compilation
toolchain, `cargo-lambda` or zig. The build is reproducible because the compiler, base image and dependencies
(`--locked`) are all pinned.

## Considered options

- **A zip built with `cargo-lambda`.** Smaller and faster to cold start, but it needs `cargo-lambda`, zig and a
  cross-compilation toolchain on every machine and in CI.

## Consequences

The image is about 200 MB, mostly the base image, so cold starts are slightly slower than a zip. Building ARM64 on an
x86 CI runner runs under emulation and is slow; use an ARM runner for deploys. Local development uses plain `cargo run`.
