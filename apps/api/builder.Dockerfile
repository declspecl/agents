# syntax=docker/dockerfile:1
#
# The build environment for the Lambda binary. It is not the Lambda itself: CDK runs the build inside this image and
# deploys only the resulting `bootstrap` binary as a plain zip.
#
# It is Amazon Linux 2023, the OS Lambda's provided.al2023 runtime runs on, so the binary links against the same glibc
# it will run with.
FROM public.ecr.aws/amazonlinux/amazonlinux:2023

RUN dnf install -y gcc && dnf clean all

ENV RUSTUP_HOME=/opt/rustup \
    CARGO_HOME=/opt/cargo \
    PATH=/opt/cargo/bin:$PATH
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain none

# Install the exact nightly pinned in rust-toolchain.toml now, so bundling never downloads a toolchain.
COPY rust-toolchain.toml /toolchain/rust-toolchain.toml
RUN cd /toolchain && rustup toolchain install

# CDK mounts the crate here and runs the build.
WORKDIR /asset-input
