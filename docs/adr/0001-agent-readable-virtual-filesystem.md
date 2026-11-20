---
status: accepted
---

# The site is a virtual filesystem for agents, built on llms.txt and friends

Gavin wants to point any AI agent at one domain and have it learn his setup and preferences. Agents already know how
to read `llms.txt`, `sitemap.xml`, `robots.txt` and ordinary documents, so the API fakes a read-only filesystem out of
those: an index at `/llms.txt`, directory listings at paths ending in `/`, and documents as markdown.

## Considered options

- **An MCP server only.** Structured, but every agent must be configured with it, which defeats "just point it at the
  domain". It stays a possible addition over the same content.
- **A conventional website.** Built for humans and browsers; agents scrape HTML poorly.
- **A single large document.** Simple, but it cannot be walked, cached per part or grown without bloating every read.

## Consequences

The API serves content and nothing else: stateless, read-only, no accounts. Indexes are generated from the content tree
so they cannot drift. Content is written for a model reading it cold. See [`docs/CONTENT.md`](../CONTENT.md).
