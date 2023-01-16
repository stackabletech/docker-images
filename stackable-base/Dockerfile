FROM registry.access.redhat.com/ubi8/ubi-minimal:8.7@sha256:e7ac72a1704622c46ca2f21f6d2aac3770b9408fa3add45f9d37008dad8f24ec

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
