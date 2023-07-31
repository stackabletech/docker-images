# syntax=docker/dockerfile:1

FROM stackable/image/stackable-base

ARG PRODUCT
ARG RPM_RELEASE

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

# Init Jobs/Pods often start a Vector Sidecar Container which collects the logs.
# As soon as an Init Container is done it'll need to tell the Vector sidecar that it can now also stop
# This happens by writing a "shutdown file" in a shared volume
# See https://github.com/stackabletech/airflow-operator/blob/23.4.1/rust/operator-binary/src/airflow_db_controller.rs#L269 for an example
# The Vector container waits for this file to appear and this waiting happens using `inotifywait` which comes from the `inotify-tools` package
RUN rpm --install https://dl.fedoraproject.org/pub/epel/epel-release-latest-8.noarch.rpm  && \
    microdnf update && \
    microdnf --assumeyes install inotify-tools && \
    microdnf clean all && \
    rpm --install "https://repo.stackable.tech/repository/packages/vector/vector-${PRODUCT}-${RPM_RELEASE}.$(arch).rpm" && \
    mkdir /licenses && \
    cp /usr/share/licenses/vector-${PRODUCT}/LICENSE /licenses/VECTOR_LICENSE && \
    # Create the directory /stackable/vector/var.
    # This directory is set by operator-rs in the parameter `data_dir`
    # of the Vector configuration. The directory is used for persisting
    # Vector state, such as on-disk buffers, file checkpoints, and more.
    # Vector needs write permissions.
    mkdir --parents /stackable/vector/var && \
    chown --recursive stackable:stackable /stackable/
