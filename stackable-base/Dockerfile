FROM registry.access.redhat.com/ubi8/ubi-minimal@sha256:987ae81ce046652ee4a2c3df54dad5e82faa1b078dab5d09f7cfaae11784ed30



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
