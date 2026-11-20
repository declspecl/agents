# syntax=docker/dockerfile:1
FROM rust:1-bookworm AS build
# rustup installs the nightly pinned in rust-toolchain.toml on first cargo use.
WORKDIR /app
# Build dependencies first so source edits reuse the cached layer.
COPY rust-toolchain.toml Cargo.toml Cargo.lock build.rs ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs && cargo build --release --locked && rm -rf src
COPY src ./src
# Embedded into the binary at compile time, so they are build inputs.
COPY content ./content
COPY templates ./templates
RUN touch src/main.rs && cargo build --release --locked

FROM public.ecr.aws/lambda/provided:al2023
COPY --from=build /app/target/release/api ${LAMBDA_RUNTIME_DIR}/bootstrap
CMD ["handler"]
