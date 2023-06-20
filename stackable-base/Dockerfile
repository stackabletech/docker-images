FROM registry.access.redhat.com/ubi8/ubi-minimal@sha256:621f5245fb3e8597a626163cdf1229e1f8311e07ab71bb1e9332014b51c59f9c 

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
