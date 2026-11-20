# Content and the virtual filesystem

> **Status:** the endpoints, file-based routing and baseline content below are built. Items marked _Planned_ are not.
> Update this file as each piece ships.

## Purpose

An agent given only `https://agents.gavindhondt.com` should be able to learn Gavin's setup, preferences and working
style by reading, with no prior knowledge and no special client. The API fakes a read-only filesystem over HTTP using
formats agents already know.

The audience is an agent first. Write content to be useful to a model reading it cold: explicit, self-contained,
plain markdown, with the reason behind each preference and not only the preference.

## Endpoints

| Path               | Returns                                                                               |
| ------------------ | ------------------------------------------------------------------------------------- |
| `/llms.txt`        | The entry point: a short description of the site and links to every top-level section |
| `/llms-full.txt`   | Every document concatenated, for an agent that wants one fetch                        |
| `/sitemap.xml`     | Every document URL, generated from the content tree                                   |
| `/robots.txt`      | Crawl policy. Everything is allowed; it also points at the sitemap and `llms.txt`     |
| `/<dir>/`          | A **directory** listing: its documents and subdirectories, as markdown with links     |
| `/<dir>/<name>.md` | A **document**, as markdown                                                           |
| `/health`          | `{"status":"ok"}`. Built                                                              |

The filesystem metaphor is the point: an agent reads `/llms.txt`, follows a link to a directory, lists it, and reads
documents, the way it would use `ls` and `cat`. Every response links onward, so the tree is walkable without outside
knowledge.

## Rules

- **Markdown by default.** A document is returned as `text/markdown; charset=utf-8`. JSON is available through the
  `Accept` header for anything programmatic. _Planned._
- **Directories end in `/`.** A request for a directory without the slash redirects to the slash form.
- **Generated, not hand-written, indexes.** `llms.txt`, `llms-full.txt`, `sitemap.xml` and every directory listing are
  built from the content tree, so they cannot drift from it.
- **Cacheable.** _Planned._ Content only changes on deploy, so responses will carry `ETag` and long-lived
  `Cache-Control`, and the API will answer conditional requests with 304.
- **Read-only.** Only `GET` and `HEAD`. Anything else is 405.
- **Unknown paths are 404** with a short markdown body that links back to `/llms.txt`, so a lost agent can recover.
- **No state.** No database, sessions or cookies. Two requests for the same path return the same bytes until a deploy.

## Content

Content is markdown in `apps/api/content/`, embedded into the binary at build time
([ADR 0002](adr/0002-content-embedded-from-the-repo.md)), so nothing is read from disk at runtime. **The directory tree
under `content/` is the URL tree.** To add a document, add a file; there is no route, template or registry to edit.
Nested directories work to any depth: `content/languages/rust/ownership.md` is served at
`/languages/rust/ownership.md` and `/languages/rust/` lists it.

A source file is TOML front matter between `+++` fences, then markdown that starts with a level-one heading:

```markdown
+++
title = "Code style"
summary = "Naming, structure and formatting preferences."
+++

# Code style

...
```

`title` and `summary` are required and become the link text and description in `llms.txt` and the listings. File and
directory names are lowercase letters, digits and single hyphens, and documents end in `.md`. A malformed or misnamed
file fails startup and the test suite, listing every problem, so it never reaches a request.

All content is public ([ADR 0003](adr/0003-all-content-is-public.md)). **Never put a secret, token, private address or
anything personal that should not be world-readable in `content/`.** `robots.txt` is a request to well-behaved
crawlers, not access control.

## Open decisions

- Whether front matter should become optional, deriving the title from the first heading and the summary from the first
  paragraph.
- Whether a directory may have an intro document that its listing shows.
- Whether `/llms-full.txt` includes every document or only a curated set.
- Whether to add an MCP interface over the same content for agents that prefer tool calls to crawling.
- The initial content outline.
