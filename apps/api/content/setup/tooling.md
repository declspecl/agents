+++
title = "Tooling"
summary = "The languages, package managers, build tools and deployment approach Gavin uses."
+++

# Tooling

## Machine and runtimes

- macOS on Apple Silicon.
- Runtime versions are managed with mise and pinned per project in `mise.toml`. Prefer mise over `.nvmrc` or
  `.tool-versions`.
- Rust toolchains are managed by rustup and pinned in `rust-toolchain.toml`, not in mise.

## Repositories

- Monorepos use pnpm workspaces with a strict `catalog:` for dependency versions, and Turborepo to run tasks.
- A shared tools package per concern (ESLint, Prettier, TypeScript config) is preferred over copying configuration.
- Husky and lint-staged run formatting on commit.

## Languages

- TypeScript with strict compiler options, formatted by Prettier.
- Rust on a pinned nightly, with clippy denying `unwrap`, `expect` and `panic` outside tests.

## Deployment

- AWS, described with CDK in TypeScript.
- Reproducible deploys matter more than small artifacts. Lambdas that need a native toolchain are built in Docker as
  container images, so a deploy needs Docker and nothing exotic installed locally.
- Services are stateless where possible.
