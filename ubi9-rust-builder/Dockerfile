# syntax=docker/dockerfile:1.8.1@sha256:e87caa74dcb7d46cd820352bfea12591f3dba3ddc4285e19c7dcd13359f7cefd

# Manifest list digest because of multi architecture builds ( https://www.redhat.com/architect/pull-container-image#:~:text=A%20manifest%20list%20exists%20to,system%20on%20a%20specific%20architecture )
FROM registry.access.redhat.com/ubi9-minimal@sha256:a7d837b00520a32502ada85ae339e33510cdfdbc8d2ddf460cc838e12ec5fa5a AS builder

LABEL maintainer="Stackable GmbH"

# This SHOULD to be kept in sync with operator-templating and other tools to reduce build times
ENV RUST_DEFAULT_TOOLCHAIN_VERSION=1.80.0
ENV CARGO_CYCLONEDX_CRATE_VERSION=0.4.0
ENV CARGO_AUDITABLE_CRATE_VERSION=0.6.4
ENV PROTOC_VERSION=27.2

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
  && curl --location --output protoc.zip "https://repo.stackable.tech/repository/packages/protoc/protoc-${PROTOC_VERSION}-linux-${ARCH}.zip" \
  && unzip protoc.zip \
  && rm protoc.zip
ENV PROTOC=/opt/protoc/bin/protoc
WORKDIR /

# IMPORTANT
# If you change the toolchain version here, make sure to also change the "rust_version"
# property in operator-templating/config/rust.yaml
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain $RUST_DEFAULT_TOOLCHAIN_VERSION \
&& . "$HOME/.cargo/env" && cargo --quiet install cargo-cyclonedx@$CARGO_CYCLONEDX_CRATE_VERSION cargo-auditable@$CARGO_AUDITABLE_CRATE_VERSION

# Build artifacts will be available in /app.
RUN mkdir /app

COPY shared/copy_artifacts.sh /

ONBUILD WORKDIR /src
ONBUILD COPY . /src

ONBUILD RUN . "$HOME/.cargo/env" && cargo auditable --quiet build --release --workspace && cargo cyclonedx --output-pattern package --all --output-cdx

# Copy the "interesting" files into /app.
ONBUILD RUN find /src/target/release \
                -regextype egrep \
                # The interesting binaries are all directly in ${BUILD_DIR}.
                -maxdepth 1 \
                # Well, binaries are executable.
                -executable \
                # Well, binaries are files.
                -type f \
                # Filter out tests.
                ! -regex ".*\-[a-fA-F0-9]{16,16}$" \
                # Copy the matching files into /app.
                -exec /copy_artifacts.sh {} \;

ONBUILD RUN echo "The following files will be copied to the runtime image: $(ls /app)"
