# 8.8-1014 as of 2023-07-20
FROM registry.access.redhat.com/ubi8/ubi-minimal@sha256:e52fc1de73dc2879516431ff1865e0fb61b1a32f57b6f914bdcddb13c62f84e6
# intentionally unused
ARG PRODUCT

RUN microdnf update && \
    microdnf --assumeyes install shadow-utils && \
    groupadd --gid 1000 --system stackable && \
    useradd --gid stackable --uid 1000 --system stackable -d /stackable && \
    mkdir /stackable && \
    chown stackable:stackable /stackable && \
    microdnf remove shadow-utils && \
    microdnf clean all && \
    echo "alias ll='ls -alF --color=auto'" >> /etc/profile.d/00-aliases.sh
