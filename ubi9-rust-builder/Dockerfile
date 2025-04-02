# syntax=docker/dockerfile:1.10.0@sha256:865e5dd094beca432e8c0a1d5e1c465db5f998dca4e439981029b3b81fb39ed5
# check=error=true

# Find the latest version at https://catalog.redhat.com/software/containers/ubi9/ubi-minimal/615bd9b4075b022acc111bf5?container-tabs=gti
# IMPORTANT: Be sure to use the Manifest List Digest for multi-arch support
FROM registry.access.redhat.com/ubi9/ubi-minimal@sha256:c0e70387664f30cd9cf2795b547e4a9a51002c44a4a86aa9335ab030134bf392 AS builder

LABEL maintainer="Stackable GmbH"

# This SHOULD be kept in sync with operator-templating and other tools to reduce build times
# Find the latest version here: https://doc.rust-lang.org/stable/releases.html
# renovate: datasource=github-releases packageName=rust-lang/rust
ENV RUST_DEFAULT_TOOLCHAIN_VERSION=1.84.1
# Find the latest version here: https://crates.io/crates/cargo-cyclonedx
# renovate: datasource=crate packageName=cargo-cyclonedx
ENV CARGO_CYCLONEDX_CRATE_VERSION=0.5.5
# Find the latest version here: https://crates.io/crates/cargo-auditable
# renovate: datasource=crate packageName=cargo-auditable
ENV CARGO_AUDITABLE_CRATE_VERSION=0.6.4
# Find the latest version here: https://github.com/protocolbuffers/protobuf/releases
# Upload any newer version to nexus with ./.scripts/upload_new_protoc_version.sh
# renovate: datasource=github-releases packageName=protocolbuffers/protobuf
ENV PROTOC_VERSION=27.3

# Sets the default shell to Bash with strict error handling and robust pipeline processing.
# "-e": Exits immediately if a command exits with a non-zero status
# "-u": Treats unset variables as an error, preventing unexpected behavior from undefined variables.
# "-o pipefail": Causes a pipeline to return the exit status of the last command in the pipe that failed, ensuring errors in any part of a pipeline are not ignored.
# "-c": Allows the execution of commands passed as a string
# This is automatically inherited in all other Dockerfiles that use this unless it is overwritten
SHELL ["/bin/bash", "-euo", "pipefail", "-c"]

# We configure microdnf to not install weak dependencies in this file
# Not doing this caused the content of images to become unpredictable because
# based on which packages get updated by `microdnf update` new weak dependencies
# might be installed that were not present earlier (the ubi base image doesn't
# seem to install weak dependencies)
# This also affects the packages that are installed in our Dockerfiles (java as prime
# example).
# https://github.com/stackabletech/docker-images/pull/533
COPY stackable-base/stackable/dnf.conf /etc/dnf/dnf.conf

# Update image and install everything needed for Rustup & Rust
RUN microdnf update \
  && microdnf install \
    clang \
    cmake \
    curl-minimal \
    findutils \
    gcc \
    gcc-c++ \
    # krb5 needed for secret-operator
    krb5-devel \
    krb5-libs \
    libkadm5 \
    make \
    openssl-devel \
    pkg-config \
    systemd-devel \
    unzip \
  && microdnf clean all \
  && rm -rf /var/cache/yum

# Container Storage Interface is defined using GRPC/Protobuf, our operators that use it (secret-operator/listener-operator) require
# protoc via Prost (https://github.com/tokio-rs/prost).
WORKDIR /opt/protoc
# Prost does not document which version of protoc it expects (https://docs.rs/prost-build/0.12.4/prost_build/), so this should be the latest upstream version
# (within reason).
RUN ARCH=$(arch | sed 's/^aarch64$/aarch_64/') \
  && curl --fail --location --output protoc.zip "https://repo.stackable.tech/repository/packages/protoc/protoc-${PROTOC_VERSION}-linux-${ARCH}.zip" \
  && unzip protoc.zip \
  && rm protoc.zip
ENV PROTOC=/opt/protoc/bin/protoc
WORKDIR /

# IMPORTANT
# If you change the toolchain version here, make sure to also change the "rust_version"
# property in operator-templating/config/rust.yaml
RUN <<EOF
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain "$RUST_DEFAULT_TOOLCHAIN_VERSION"
. "$HOME/.cargo/env"
cargo install --quiet --locked "cargo-cyclonedx@$CARGO_CYCLONEDX_CRATE_VERSION" "cargo-auditable@$CARGO_AUDITABLE_CRATE_VERSION"
EOF

# Build artifacts will be available in /app.
RUN mkdir /app

COPY shared/copy_artifacts.sh /

ONBUILD WORKDIR /src
ONBUILD COPY . /src

ONBUILD RUN <<EOF
. "$HOME/.cargo/env"
cargo auditable --quiet build --release --workspace
cargo cyclonedx --all --spec-version 1.5 --describe binaries

# -maxdepth 1: The interesting binaries are all directly in ${BUILD_DIR}.
# -regex filters out tests
# - exec copies matching files to /app
find /src/target/release \
  -regextype egrep \
  -maxdepth 1 \
  -executable \
  -type f \
  ! -regex ".*\-[a-fA-F0-9]{16,16}$" \
  -exec /copy_artifacts.sh {} \;

echo "The following files will be copied to the runtime image: $(ls /app)"
EOF
