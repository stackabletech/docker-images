# syntax=docker/dockerfile:1.15.1@sha256:9857836c9ee4268391bb5b09f9f157f3c91bb15821bb77969642813b0d00518d
# check=error=true

FROM stackable/image/stackable-base

ARG PRODUCT
ARG RPM_RELEASE
ARG INOTIFY_TOOLS
ARG TARGETARCH
ARG STACKABLE_USER_UID

# Init Jobs/Pods often start a Vector Sidecar Container which collects the logs.
# As soon as an Init Container is done it'll need to tell the Vector sidecar that it can now also stop
# This happens by writing a "shutdown file" in a shared volume
# See https://github.com/stackabletech/airflow-operator/blob/23.4.1/rust/operator-binary/src/airflow_db_controller.rs#L269 for an example
# The Vector container waits for this file to appear and this waiting happens using `inotifywait` which comes from the `inotify-tools` package
RUN <<EOF
ARCH="${TARGETARCH/amd64/x86_64}"
ARCH="${ARCH/arm64/aarch64}"
rpm --install \
    "https://repo.stackable.tech/repository/packages/vector/vector-${PRODUCT}-${RPM_RELEASE}.${ARCH}.rpm" \
    "https://repo.stackable.tech/repository/packages/inotify-tools/inotify-tools-${INOTIFY_TOOLS}.${ARCH}.rpm"
mkdir /licenses
cp /usr/share/licenses/vector-${PRODUCT}/LICENSE /licenses/VECTOR_LICENSE

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
