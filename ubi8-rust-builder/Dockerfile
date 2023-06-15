FROM registry.access.redhat.com/ubi8/ubi-minimal:8.7@sha256:3e1adcc31c6073d010b8043b070bd089d7bf37ee2c397c110211a6273453433f AS builder
LABEL maintainer="Stackable GmbH"

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

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
# property in operator-templating/repositories.yaml
# hadolint ignore=SC1091
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.68.2 \
 && . "$HOME/.cargo/env" && cargo install cargo-cyclonedx@0.3.7 cargo-auditable@0.6.1

# Build artifacts will be available in /app.
RUN mkdir /app

COPY copy_artifacts.sh /

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
