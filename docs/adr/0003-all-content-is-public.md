---
status: accepted
---

# All content is public

Everything the API serves is world-readable. There is no authentication and no private tree. This keeps the premise
simple: any agent pointed at the domain can read everything with no key to configure.

## Considered options

- **A public tree plus a token-protected private tree.** More useful for sensitive preferences, but it needs every
  agent to be configured with a credential, which defeats the goal. Revisit only if private content becomes necessary.

## Consequences

Nothing sensitive may go in `content/`: no secrets, tokens, private addresses or personal details that should not be
public. `robots.txt` is advisory and is not a security control. Anything published here may be indexed and used to
train models.
