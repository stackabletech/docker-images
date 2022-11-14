FROM docker.stackable.tech/sandbox/logging/vector:0.25.1-stackable1.0.0@sha256:5da88737b547318d8ce0402c55f3ea6e204a14fe7ab68e4e1c923cdb9edd32f7

ARG PRODUCT
ARG RELEASE="1"
ARG _SECURITY_PATH

LABEL name="Stackable image for OpenJDK" \
      maintainer="info@stackable.de" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable OpenJDK base image." \
      description="This image is the base image for all Stackable Java product images."

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf install java-${PRODUCT}-openjdk-headless --nodocs && \
    microdnf clean all

COPY java-base/licenses /licenses

ENV JAVA_HOME=/usr/lib/jvm/jre-${PRODUCT}

# Some uses (such as Kerberos or TLS client hostname verification) requires
# up-to-date reverse DNS information, especially when workloads migrate
# between Nodes and/or switch Pod IPs
RUN printf "networkaddress.cache.ttl=0\nnetworkaddress.cache.negative.ttl=0\n" >> $_SECURITY_PATH
