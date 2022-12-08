# Build stage

FROM docker.stackable.tech/stackable/stackable-base:1.0.0-stackable1.0.0@sha256:566607cbbfc78be4004dd4ecd06a2673788b56eaa3c330e3e2953902c5b6b94c AS builder

ARG PRODUCT

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf --assumeyes install gzip-1.9 tar-2:1.30 && \
    microdnf clean all && \
    curl "https://repo.stackable.tech/repository/packages/vector/vector-${PRODUCT}-$(arch)-unknown-linux-gnu.tar.gz" | \
    tar x -o -z -C /opt && \
    mkdir --parents /build/licenses && \
    cp "/opt/vector-$(arch)-unknown-linux-gnu/LICENSE" /build/licenses/VECTOR_LICENSE && \
    mkdir --parents /build/stackable/vector/ && \
    cp --recursive "/opt/vector-$(arch)-unknown-linux-gnu/bin/" /build/stackable/vector/ && \
    mkdir --parents /build/stackable/vector/var && \
    chown --recursive stackable:stackable /build/stackable/

# Final image

FROM docker.stackable.tech/stackable/stackable-base:1.0.0-stackable1.0.0@sha256:566607cbbfc78be4004dd4ecd06a2673788b56eaa3c330e3e2953902c5b6b94c

COPY --from=builder /build /
