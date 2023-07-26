# syntax=docker/dockerfile:1
FROM stackable/image/vector

ARG PRODUCT
ARG RELEASE="1"
ARG _SECURITY_PATH

LABEL name="Stackable image for OpenJDK" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable OpenJDK base image." \
      description="This image is the base image for all Stackable Java product images."

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf install \
        # Obviously needed to run java programms
        java-${PRODUCT}-openjdk-headless \
        # Needed, because otherwise e.g. Zookeeper fails with
        # Caused by: java.io.FileNotFoundException: /usr/lib/jvm/java-11-openjdk-11.0.20.0.8-2.el8.x86_64/lib/tzdb.dat (No such file or directory)
        tzdata-java \
        --nodocs && \
    microdnf clean all

COPY java-base/licenses /licenses

ENV JAVA_HOME=/usr/lib/jvm/jre-${PRODUCT}

# Some uses (such as Kerberos or TLS client hostname verification) requires
# up-to-date reverse DNS information, especially when workloads migrate
# between Nodes and/or switch Pod IPs
RUN printf "networkaddress.cache.ttl=0\nnetworkaddress.cache.negative.ttl=0\n" >> $_SECURITY_PATH
