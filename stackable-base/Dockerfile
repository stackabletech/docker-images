FROM registry.access.redhat.com/ubi8/ubi-minimal:8.6@sha256:33931dce809712888d1a8061bfa676963f517daca993984afed3251bc1fb5987

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
