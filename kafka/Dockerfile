FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf install java-11-openjdk-headless --nodocs && \
    microdnf install tar gzip && \
    microdnf install shadow-utils && \
    microdnf clean all

ENV JAVA_HOME=/usr/lib/jvm/jre-11

COPY stackable /stackable
RUN groupadd -r stackable --gid=1000 && \
    useradd -r -g stackable --uid=1000 stackable && \
    chown -R stackable:stackable /stackable

USER stackable

ARG PRODUCT
ARG SCALA
ARG OPA_AUTHORIZER

WORKDIR /stackable
RUN curl -L https://repo.stackable.tech/repository/packages/kafka/kafka_${SCALA}-${PRODUCT}.tgz | tar -xzC . && \
    ln -s /stackable/kafka_${SCALA}-${PRODUCT} /stackable/kafka

RUN curl -L https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-0.16.1.jar \
    -o /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar

RUN curl -L https://repo.stackable.tech/repository/packages/kafka-opa-authorizer/opa-authorizer-${OPA_AUTHORIZER}-all.jar \
    -o /stackable/kafka/libs/opa-authorizer-${OPA_AUTHORIZER}-all.jar

# Mitigate CVE-2021-44228 (Log4Shell)
ENV LOG4J_FORMAT_MSG_NO_LOOKUPS=true

WORKDIR /stackable/kafka
CMD ["bin/kafka-server-start.sh", "/stackable/kafka/config/server.properties"]
