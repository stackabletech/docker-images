# syntax=docker/dockerfile:1.8.1@sha256:e87caa74dcb7d46cd820352bfea12591f3dba3ddc4285e19c7dcd13359f7cefd

# Ignoring DL3038 globally because set `assumeyes=True` in dnf.conf in our base image
# Ignoring DL4006 globally because we inherit the SHELL from our base image
# hadolint global ignore=DL3038,DL4006

# Not tagging base image because this is an intermediate image
# hadolint ignore=DL3006
FROM stackable/image/stackable-base
ARG PRODUCT

# We don't pin versions
# hadolint ignore=DL3041
RUN microdnf update && \
    microdnf install \
        # used to unpack the statsd_exporter source
        tar \
        gzip \
        # used to build statsd_exporter
        golang && \
    microdnf clean all && \
    rm -rf /var/cache/yum && \
    mkdir -p /statsd_exporter
WORKDIR /statsd_exporter

# hadolint ignore=DL3003
RUN curl --fail -L "https://repo.stackable.tech/repository/packages/statsd_exporter/statsd_exporter-${PRODUCT}.src.tar.gz" | \
    tar -xzC . && cd "statsd_exporter-${PRODUCT}" && go build -o ../statsd_exporter && cd .. && \
    rm -rf "statsd_exporter-${PRODUCT}"