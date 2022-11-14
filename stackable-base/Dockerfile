FROM registry.access.redhat.com/ubi8/ubi-minimal:8.6@sha256:c5ffdf5938d73283cec018f2adf59f0ed9f8c376d93e415a27b16c3c6aad6f45

# intentionally unused
ARG PRODUCT

RUN microdnf update && \
    microdnf --assumeyes install shadow-utils && \
    groupadd --gid 1000 --system stackable && \
    useradd --gid stackable --uid 1000 --system stackable && \
    mkdir /stackable && \
    chown stackable:stackable /stackable && \
    microdnf remove shadow-utils && \
    microdnf clean all
