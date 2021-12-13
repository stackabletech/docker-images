FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf install java-11-openjdk-headless --nodocs && \
    microdnf install tar gzip python3 && \
    microdnf install shadow-utils && \
    microdnf clean all

ENV JAVA_HOME=/usr/lib/jvm/jre-11

COPY stackable /stackable
RUN groupadd -r stackable --gid=1000 && \
    useradd -r -g stackable --uid=1000 stackable && \
    chown -R stackable:stackable /stackable

RUN alternatives --set python /usr/bin/python3

USER stackable

ARG PRODUCT

WORKDIR /stackable
RUN curl -L https://repo.stackable.tech/repository/packages/trino-server/trino-server-${PRODUCT}.tar.gz | tar -xzC . && \
    ln -s /stackable/trino-server-${PRODUCT} /stackable/trino-server

RUN curl https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-0.16.1.jar \
    -o /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar && \
    chmod -x /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar

WORKDIR /stackable/trino-server/plugin
RUN curl -L https://repo.stackable.tech/repository/packages/trino-opa-authorizer/trino-opa-authorizer-0.1.0.tar.gz | tar -xzC .

# Mitigate CVE-2021-44228 (Log4Shell)
ENV LOG4J_FORMAT_MSG_NO_LOOKUPS=true

WORKDIR /stackable/trino-server
CMD ["bin/launcher", "run", "--etc-dir=/stackable/conf"]
