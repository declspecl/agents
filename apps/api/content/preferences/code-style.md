+++
title = "Code style"
summary = "Naming, structure, formatting and testing preferences that apply across projects."
+++

# Code style

## Naming

- Variables, fields and constants are nouns. Functions and methods are verbs.
- Conversion accessors keep a language's own `as_`, `to_` and `into_` convention rather than becoming verbs.
- Name things for what they are. "Environment" means the process's variables and the typed model read from them; do not
  call that "config".

## Structure

- Organize files deliberately. Give each concept its own module or file and do not pile unrelated things together.
  Being lazy about layout is the most common way to disappoint.
- Prefer many small, cohesive files over a few large ones.
- Tests live in their own location, not inline in the middle of source: integration tests in a `tests/` directory, unit
  tests in sibling test files.

## Modeling

- Model data strongly. Parse input once, at the boundary, into types that cannot hold invalid values (a validated path,
  a parsed address), and pass those types around rather than strings.
- Be explicit. Every environment variable is a named, documented entry in one registry, and nothing reads the
  environment anywhere else.
- Report every problem at once when validating, not only the first.

## Formatting

- Run the formatter and linter; they decide style. Do not argue with them in review.
- Rust uses a pinned nightly toolchain and its nightly rustfmt options. TypeScript uses Prettier with four-space indents
  and a 140-column width.

## Errors

- Return errors instead of panicking. `unwrap` and `expect` are for tests.
- Never swallow an error silently.
