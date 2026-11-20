---
status: accepted
---

# Content lives in the repo and is embedded at build time

The markdown the API serves is kept in `apps/api/content/` and compiled into the binary (`include_dir!`), not read from S3 at request time.
Content is small, changes slowly, and benefits from review and history, so git is the right store. Embedding means no
AWS permissions, no read latency, no cold-start network call and no failure mode where content is missing.

## Considered options

- **S3.** Lets content change without a deploy. Adds IAM, latency and a runtime dependency, for a need that does not
  exist yet.

## Consequences

Content changes ship on deploy. If content grows large, or editing without a deploy becomes worth the cost, move the
source behind the same interface and revisit this ADR; the routes should not need to change.
