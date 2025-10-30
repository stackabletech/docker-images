# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

FROM local-image/stackable-devel AS vector-builder

ARG PRODUCT_VERSION
ARG RELEASE_VERSION
ARG STACKABLE_USER_UID
ARG PROTOC_VERSION

RUN <<EOF
microdnf update
microdnf install \
    `# vector docs say we need these (trying automake instead of autotools)` \
    cmake \
    automake \
    `# openssl libs and related packages required by the build` \
    perl \
    findutils \
    openssl-devel \
    pkg-config \
    `# tar needed to create the source code snapshot before building the Rust code` \
    tar \
    `# needed for rdkafka-sys` \
    cyrus-sasl-devel
microdnf clean all
rm -rf /var/cache/yum
EOF

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

WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 vector/stackable/patches/patchable.toml /stackable/src/vector/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 vector/stackable/patches/${PRODUCT_VERSION} /stackable/src/vector/stackable/patches/${PRODUCT_VERSION}

# Build artifacts will be available in /app.
RUN mkdir /app

# This script is designed for operator, and their source path.
# So we can't use it. Instead we use a modified version.
# COPY shared/copy_artifacts.sh /
COPY vector/copy_artifacts.sh /

RUN <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout vector ${PRODUCT_VERSION})"

NEW_VERSION="${PRODUCT_VERSION}-stackable${RELEASE_VERSION}"

# Create snapshot of the source code including custom patches
tar -czf /stackable/vector-${NEW_VERSION}-src.tar.gz .

# rust build here?
. "$HOME/.cargo/env"
# cargo auditable --quiet build --release --workspace
cargo auditable --quiet build --release

# Generate SBOMs and copy them to /app (via a script)
cargo cyclonedx --all --spec-version 1.5 --describe binaries

# -maxdepth 1: The interesting binaries are all directly in ${BUILD_DIR}.
# -regex filters out tests
# - exec copies matching files to /app
find target/release \
  -regextype egrep \
  -maxdepth 1 \
  -executable \
  -type f \
  ! -regex ".*\-[a-fA-F0-9]{16,16}$" \
  -exec /copy_artifacts.sh {} \;

echo "The following files will be copied to the runtime image: $(ls /app)"

# Set correct permissions
chmod -R g=u /stackable
EOF

FROM local-image/stackable-base

ARG PRODUCT_VERSION
ARG RPM_RELEASE
ARG INOTIFY_TOOLS
ARG TARGETARCH
ARG STACKABLE_USER_UID

COPY --chown=${STACKABLE_USER_UID}:0 opa/licenses /licenses
COPY --from=vector-builder --chown=${STACKABLE_USER_UID}:0 /app/* /usr/local/bin/

# Init Jobs/Pods often start a Vector Sidecar Container which collects the logs.
# As soon as an Init Container is done it'll need to tell the Vector sidecar that it can now also stop
# This happens by writing a "shutdown file" in a shared volume
# See https://github.com/stackabletech/airflow-operator/blob/23.4.1/rust/operator-binary/src/airflow_db_controller.rs#L269 for an example
# The Vector container waits for this file to appear and this waiting happens using `inotifywait` which comes from the `inotify-tools` package
RUN <<EOF
ARCH="${TARGETARCH/amd64/x86_64}"
ARCH="${ARCH/arm64/aarch64}"
rpm --install \
    "https://repo.stackable.tech/repository/packages/inotify-tools/inotify-tools-${INOTIFY_TOOLS}.${ARCH}.rpm"

# Create the directory /stackable/vector/var.
# This directory is set by operator-rs in the parameter `data_dir`
# of the Vector configuration. The directory is used for persisting
# Vector state, such as on-disk buffers, file checkpoints, and more.
# Vector needs write permissions.
mkdir --parents /stackable/vector/var
chown --recursive ${STACKABLE_USER_UID}:0 /stackable/
# Set correct permissions
chmod -R g=u /stackable
EOF
