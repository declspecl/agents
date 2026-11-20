# agents

A public website at `agents.gavindhondt.com` that any AI agent can be pointed at to learn about Gavin's setup and
preferences. A stateless Rust REST API on AWS Lambda presents markdown as a virtual filesystem, using `llms.txt`,
`sitemap.xml` and `robots.txt`.

**Start with [`AGENTS.md`](AGENTS.md)**: purpose, status, layout, commands and conventions. The content design is in
[`docs/CONTENT.md`](docs/CONTENT.md) and the decisions are in [`docs/adr/`](docs/adr/).

## Quick start

Requires Node 24 (`mise install`), pnpm, Docker and Rust.

```sh
pnpm install
pnpm api:dev                             # http://127.0.0.1:3000
cp apps/cdk/.env.example apps/cdk/.env.local   # fill in AWS_ACCOUNT_ID
pnpm cdk:bootstrap && pnpm cdk:deploy
```

The `gavindhondt.com` hosted zone must exist in Route 53 in the target account.
