FROM registry.access.redhat.com/ubi8/ubi-minimal:8.5
LABEL maintainer="Stackable GmbH"

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

ARG PRODUCT
ARG _SECURITY_PATH

RUN microdnf update && \
    microdnf install java-${PRODUCT}-openjdk-headless --nodocs && \
    microdnf clean all

ENV JAVA_HOME=/usr/lib/jvm/jre-${PRODUCT}

RUN echo "networkaddress.cache.ttl=5" >> $_SECURITY_PATH
