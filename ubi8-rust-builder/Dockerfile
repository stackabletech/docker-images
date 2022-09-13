FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

# Update image and install everything needed for Rustup & Rust
RUN microdnf update --disablerepo=* --enablerepo=ubi-8-appstream --enablerepo=ubi-8-baseos --enablerepo=ubi-8-baseos -y \
  && rm -rf /var/cache/yum \
  && microdnf install --disablerepo=* --enablerepo=ubi-8-appstream --enablerepo=ubi-8-baseos curl findutils gcc gcc-c++ make cmake openssl-devel pkg-config systemd-devel unzip perl -y \
  && rm -rf /var/cache/yum

WORKDIR /opt/protoc
RUN PROTOC_VERSION=21.5 \
  ARCH=$(arch | sed 's/^aarch64$/aarch_64/') \
  && curl --location --output protoc.zip "https://repo.stackable.tech/repository/packages/protoc/protoc-${PROTOC_VERSION}-linux-${ARCH}.zip" \
  && unzip protoc.zip \
  && rm protoc.zip
ENV PROTOC=/opt/protoc/bin/protoc
WORKDIR /

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
 && . $HOME/.cargo/env \
 && rustup toolchain install stable \
 && rustup default stable

# Build artifacts will be available in /app.
RUN mkdir /app

ONBUILD WORKDIR /src
ONBUILD COPY . /src

# Getting target specific things for cross-compiling
ONBUILD RUN \
  if [ $(arch) = "aarch64" ]; then \
  . $HOME/.cargo/env && rustup target add aarch64-unknown-linux-gnu; \
  else \
  . $HOME/.cargo/env && rustup target add x86_64-unknown-linux-gnu; \
  fi

# Setting up linker and compiler
ONBUILD ENV \ 
   . $HOME/.cargo/env && \ 
  if [ $(arch) = "aarch64" ]; then \
  CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
  CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc \
  CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++; \
  else \
  CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-linux-gnu-gcc \
  CC_X86_64_unknown_linux_gnu=x86_64-linux-gnu-gcc \
  CXX_X86_64_unknown_linux_gnu=x86_64-linux-gnu-g++; \ 
  fi

# Building platform specific binary
ONBUILD RUN \
  if [ $(arch) = "aarch64" ]; then \
  . $HOME/.cargo/env && cargo build --release --target=aarch64-unknown-linux-gnu; \
  else \
  . $HOME/.cargo/env && cargo build --release --target=x86_64-unknown-linux-gnu; \
  fi 
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
                -exec cp {} /app \;

ONBUILD RUN echo "The following files will be copied to the runtime image: $(ls /app)"
