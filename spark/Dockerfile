FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf install java-11-openjdk-headless --nodocs && \
    microdnf install tar gzip && \
    microdnf install shadow-utils && \
    # required for spark startup scripts
    microdnf install procps && \
    microdnf install hostname && \
    microdnf clean all

ENV JAVA_HOME=/usr/lib/jvm/jre-11

COPY stackable /stackable
RUN groupadd -r stackable --gid=1000 && \
    useradd -r -g stackable --uid=1000 stackable && \
    chown -R stackable:stackable /stackable

USER stackable

ARG PRODUCT
ARG HADOOP

WORKDIR /stackable
RUN curl -L https://repo.stackable.tech/repository/packages/spark/spark-${PRODUCT}-bin-hadoop${HADOOP}.tgz | tar -xzC . && \
    ln -s /stackable/spark-${PRODUCT}-bin-hadoop${HADOOP} /stackable/spark

RUN curl https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-0.16.1.jar \
    -o /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar && \
    chmod -x /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar

WORKDIR /stackable/spark
CMD ["sbin/start-master.sh"]
