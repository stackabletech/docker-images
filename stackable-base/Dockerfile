# 8.8-1014 as of 2023-07-20
FROM registry.access.redhat.com/ubi8@sha256:449da7f8f2ef6285a8445a1e31af57a97b9dae5dcf009b1629c59742c89c68c3 
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
