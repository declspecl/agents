# agents

Rust REST API Lambda served at `agents.gavindhondt.com`, deployed with AWS CDK. Turborepo + pnpm monorepo.

| Path       | What                                                                              |
| ---------- | --------------------------------------------------------------------------------- |
| `apps/api` | Rust (axum + `lambda_http`) Lambda, built by Docker into an ARM64 container image |
| `apps/cdk` | CDK app: Lambda, HTTP API, ACM cert, custom domain, Route 53 alias                |
| `tools/*`  | Shared prettier, tsconfig and eslint config                                       |

## Setup

Requires Node 24, pnpm and Docker. Deploys build the Rust image in Docker, so no Rust toolchain, cargo-lambda or zig is needed to deploy. Rust is only needed for local `cargo` dev.

```sh
pnpm install
cp apps/cdk/.env.example apps/cdk/.env   # fill in AWS_ACCOUNT_ID
pnpm api:dev                             # serves http://localhost:3000 (cargo run)
pnpm cdk:bootstrap && pnpm cdk:deploy
```

The `gavindhondt.com` hosted zone must exist in Route 53 in the target account.
