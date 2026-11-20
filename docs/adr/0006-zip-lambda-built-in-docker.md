---
status: accepted
---

# The Lambda is a plain-binary zip, compiled in Docker by CDK bundling

[ADR 0004](0004-container-image-lambda.md) deployed the API as a container image. It is now a zip containing one
`bootstrap` binary on `provided.al2023`, ARM64. CDK's asset bundling compiles the crate inside
`apps/api/builder.Dockerfile`, an Amazon Linux 2023 image with the pinned nightly, and zips only the result. A deploy
still needs Docker and nothing else, and the build is still reproducible: the OS, compiler and dependencies (`--locked`)
are pinned. Building on Amazon Linux 2023, the OS Lambda runs on, means the binary links against the glibc it runs with.

## Considered options

- **Keep the container image.** Reproducible, but the artifact was about 200 MB, almost all of it the base image, so
  cold starts were slower and every deploy pushed an image to ECR.
- **Cross-compile on the host with `cargo-zigbuild`.** The fastest local builds, but it needs the target, zig and
  cargo-zigbuild on every machine and in CI, and the output can differ between machines.

## Consequences

- The binary is about 4.8 MB with debug info stripped, so the artifact is a few MB instead of about 200 MB.
- The asset hash covers the crate's source, not the build output, so an unchanged crate is not rebuilt on every synth.
  Edits to `tests/`, `package.json` and markdown files outside the embedded content are excluded from the hash.
- Building ARM64 on an x86 CI runner runs under emulation and is slow; use an ARM runner.
- Changing the function's package type replaces it, which CloudFormation refuses for a hard-coded name. The function
  name is therefore left to CloudFormation. The log group keeps its original name, since renaming it would also
  replace it. The first deploy after this change replaces the function, and the old container image stays in the
  bootstrap ECR repository until deleted.
- The function runs with 128 MB and a 3-second timeout. The process holds about 4 MB, and a request is microseconds of
  in-memory work. Raise either if a future endpoint needs it.
