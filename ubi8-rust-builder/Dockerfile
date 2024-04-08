# syntax=docker/dockerfile:1.6.0@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021

# 8.9-1137 as of 2024-03-25
# Manifest list digest because of multi architecture builds ( https://www.redhat.com/architect/pull-container-image#:~:text=A%20manifest%20list%20exists%20to,system%20on%20a%20specific%20architecture )
FROM registry.access.redhat.com/ubi8/ubi-minimal@sha256:8bedbe742f140108897fb3532068e8316900d9814f399d676ac78b46e740e34e AS builder
LABEL maintainer="Stackable GmbH"

# Sets the default shell to Bash with strict error handling and robust pipeline processing.
# "-e": Exits immediately if a command exits with a non-zero status
# "-u": Treats unset variables as an error, preventing unexpected behavior from undefined variables.
# "-o pipefail": Causes a pipeline to return the exit status of the last command in the pipe that failed, ensuring errors in any part of a pipeline are not ignored.
# "-c": Allows the execution of commands passed as a string
# This is automatically inherited in all other Dockerfiles that use this unless it is overwritten
SHELL ["/bin/bash", "-euo", "pipefail", "-c"]

# Update image and install everything needed for Rustup & Rust
RUN microdnf update --disablerepo=* --enablerepo=ubi-8-appstream-rpms --enablerepo=ubi-8-baseos-rpms -y \
  && rm -rf /var/cache/yum \
  && microdnf install --disablerepo=* --enablerepo=ubi-8-appstream-rpms --enablerepo=ubi-8-baseos-rpms curl findutils gcc gcc-c++ make cmake openssl-devel pkg-config systemd-devel unzip clang krb5-libs libkadm5 -y \
  && rm -rf /var/cache/yum

WORKDIR /opt/protoc
RUN PROTOC_VERSION=21.5 \
  ARCH=$(arch | sed 's/^aarch64$/aarch_64/') \
  && curl --location --output protoc.zip "https://repo.stackable.tech/repository/packages/protoc/protoc-${PROTOC_VERSION}-linux-${ARCH}.zip" \
  && unzip protoc.zip \
  && rm protoc.zip
ENV PROTOC=/opt/protoc/bin/protoc
WORKDIR /

# IMPORTANT
# If you change the toolchain version here, make sure to also change the "rust_version"
# property in operator-templating/config/rust.yaml
# hadolint ignore=SC1091
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.77.0 \
 && . "$HOME/.cargo/env" && cargo install cargo-cyclonedx@0.4.0 cargo-auditable@0.6.1

# Build artifacts will be available in /app.
RUN mkdir /app

COPY ubi8-rust-builder/copy_artifacts.sh /

ONBUILD WORKDIR /src
ONBUILD COPY . /src

# hadolint ignore=SC1091
ONBUILD RUN . "$HOME/.cargo/env" && cargo auditable build --release --workspace && cargo cyclonedx --output-pattern package --all --output-cdx

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
