# syntax=docker/dockerfile:1

# Build stage
FROM stackable/image/stackable-base AS builder

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

FROM stackable/image/stackable-base

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN rpm --install https://dl.fedoraproject.org/pub/epel/epel-release-latest-8.noarch.rpm  && \
    microdnf update && \
    microdnf --assumeyes install inotify-tools && \
    microdnf clean all

COPY --from=builder /build /
