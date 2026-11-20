# agents

This repo is the **distributed agentic setup** of its owner, Gavin: one public website at `agents.gavindhondt.com`
that any AI agent can be pointed at to crawl and learn about his setup, preferences and working style. It is a
**stateless REST API** (a Rust Lambda) that presents a body of markdown as a **virtual filesystem**, using the
conventions agents already understand: `llms.txt`, `sitemap.xml`, `robots.txt` and plain path-addressed documents.

There is no website, database or user account. The API reads content and returns it. Everything it serves is public.

Read [`docs/CONTENT.md`](docs/CONTENT.md) for how the virtual filesystem works, and [`docs/adr/`](docs/adr/) for why
the main decisions were made.

## Status

| Piece                                                                         | State                              |
| ----------------------------------------------------------------------------- | ---------------------------------- |
| Monorepo, tooling, CI, CDK stack, Docker-built Rust Lambda, custom domain     | Built                              |
| Typed environment reading (Rust and CDK)                                      | Built                              |
| `GET /health`                                                                 | Built                              |
| Content tree, `llms.txt`, `llms-full.txt`, `sitemap.xml`, `robots.txt`, paths | **Planned**: see `docs/CONTENT.md` |
| Real content (preferences, setup)                                             | **Planned**                        |

Do not describe planned pieces as existing. When one ships, update this table and `docs/CONTENT.md` in the same change.

## Glossary

**Content**: the markdown documents the API serves. They live in `content/` in this repo (planned) and describe
Gavin's setup and preferences.
_Avoid_: data, files (a file is a source on disk; see _Document_)

**Document**: one unit of content, addressed by a path such as `/preferences/editor.md`.
_Avoid_: page, file, resource

**Directory**: a path ending in `/` that lists its children. It is a view the API builds, not a stored thing.
_Avoid_: folder, collection

**Virtual filesystem**: the whole path-addressed tree as an agent sees it. It is faked: there is no real filesystem
behind the API at request time, only embedded content.

**Agent**: any AI client reading the site. It is the intended reader; design for it first and for humans second.
_Avoid_: user, visitor, crawler (a crawler is one kind of agent)

## Layout

```
apps/api               Rust (axum + lambda_http) Lambda. Library crate plus a thin binary
apps/cdk               AWS CDK app: the Lambda container image, HTTP API, certificate, domain, DNS
tools/eslint           shared ESLint config
tools/prettier         shared Prettier config
tools/typescript       shared tsconfig bases
docs/                  CONTENT.md and adr/
content/               the markdown the API serves (planned; does not exist yet)
```

`apps/api` layout:

```
src/main.rs            thin entrypoint: tracing, read environment, serve, exit code
src/lib.rs             module map
src/server.rs          serve(): Lambda or local HTTP depending on Runtime
src/environment.rs     Environment, Runtime; reads the process environment once at startup
src/environment/       Variable (names and docs), EnvironmentError, Reader (private)
src/routes.rs          build_router(): registers every endpoint
src/routes/            one endpoint per file
tests/                 integration tests of the public API; unit tests live in sibling files, not inline
```

## Commands

Requires Node 24 (pinned in `mise.toml`), pnpm, Docker, and Rust (rustup reads `apps/api/rust-toolchain.toml`).

```sh
pnpm install
pnpm api:dev           # serve on http://127.0.0.1:3000 (cargo run)
pnpm test              # Rust integration tests and CDK tests
pnpm lint:check        # clippy + cargo fmt --check, and ESLint
pnpm check-types       # cargo check, and tsc
pnpm prettier:check    # formatting
pnpm cdk:synth | cdk:diff | cdk:deploy | cdk:bootstrap
```

Deploys need `apps/cdk/.env` (copy `.env.example`; `AWS_ACCOUNT_ID` is required). The `gavindhondt.com` hosted zone
must exist in Route 53 in that account.

## Conventions

**Naming.** Variables, fields and constants are **nouns**. Functions and methods are **verbs** (`build_router`,
`read_optional`, `check_health`). Conversion accessors keep Rust's `as_`/`to_`/`into_` prefixes (`Variable::as_str`).
The word **environment** means the process's variables and the typed model read from them. Do not call it config.

**Environment.** Every environment variable is a named, documented entry in one registry: `Variable` in
`apps/api/src/environment/variable.rs`, `ENVIRONMENT_VARIABLE` in `apps/cdk/src/environment/variables.ts`. Nothing
reads `std::env` or `process.env` anywhere else. Values are parsed once at startup into types (a `SocketAddr`, a branded
`AwsAccountId`), every bad variable is reported together, and a blank value counts as unset. Reading takes a lookup
function so tests never touch process state.

**Rust.**

- Nightly, pinned by date in `apps/api/rust-toolchain.toml`. `rustfmt.toml` uses nightly-only options. Bump the date
  deliberately; the Docker build reads the same file.
- `unsafe_code` is forbidden. `unwrap`, `expect`, `panic`, `unreachable`, `todo` and `unimplemented` are denied outside
  tests (`clippy.toml` allows them in tests). Return errors.
- Library plus thin binary, so `tests/` exercises the public API. Tests live in `tests/` or in sibling test files, never
  inline `#[cfg(test)]` blocks in the middle of source.
- `cargo fmt` needs two passes to converge with the nightly options. Run it twice before concluding it is clean.
- Edition 2024, so `foo.rs` beside `foo/`, never `mod.rs`.

**TypeScript.** Prettier: 4 spaces, 140 columns, trailing commas, sorted imports. ESLint forbids `any`, `as unknown`
and double assertions, and requires braces on every block. Dependencies come from the pnpm `catalog:` (strict mode).
Write tests first, then the implementation.

**Docs.** Keep these files true. A change that alters structure, commands, endpoints or a decision updates the matching
doc in the same change. A decision that bends a rule here gets an ADR.

## Deployment

The Lambda is an **ARM64 container image**: Docker compiles the Rust crate (`apps/api/Dockerfile`) and CDK builds and
pushes it during `cdk deploy`. No native Rust cross-compilation toolchain, `cargo-lambda` or zig is needed. See
[ADR 0004](docs/adr/0004-container-image-lambda.md). CI runs on GitHub Actions (`.github/workflows/ci.yml`); a pre-commit
hook runs Prettier on staged files.

## Decisions

| ADR                                                        | Decision                                                                   |
| ---------------------------------------------------------- | -------------------------------------------------------------------------- |
| [0001](docs/adr/0001-agent-readable-virtual-filesystem.md) | The site is a virtual filesystem for agents, built on llms.txt and friends |
| [0002](docs/adr/0002-content-embedded-from-the-repo.md)    | Content lives in the repo and is embedded at build time, not read from S3  |
| [0003](docs/adr/0003-all-content-is-public.md)             | All content is public; there is no auth and no private tree                |
| [0004](docs/adr/0004-container-image-lambda.md)            | The Lambda deploys as a Docker container image                             |
| [0005](docs/adr/0005-pinned-nightly-rust.md)               | Rust is nightly, pinned by date                                            |
