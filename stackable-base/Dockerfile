# 8.9-1029 as of 2023-12-01
# Manifest list digest because of multi architecture builds
FROM registry.access.redhat.com/ubi8/ubi-minimal:8.9@sha256:87bcbfedfd70e67aab3875fff103bade460aeff510033ebb36b7efa009ab6639 

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
