FROM docker.stackable.tech/stackable/vector:0.26.0-stackable1.0.0@sha256:4be0d98ca353b3ddb54212ced702504d58995b2b75fbfa07fe001fd992513692

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
