FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

RUN microdnf update && \
    microdnf install java-11-openjdk-headless --nodocs && \
    microdnf install tar gzip && \
    microdnf install shadow-utils && \
    # required for spark startup scripts
    microdnf install procps && \
    microdnf install hostname && \
    microdnf clean all

ENV JAVA_HOME=/usr/lib/jvm/jre-11

RUN groupadd -r stackable --gid=1000; \
    useradd -r -g stackable --uid=1000 stackable;

ADD stackable /stackable
RUN chown -R stackable:stackable /stackable

USER stackable

ARG PRODUCT
ARG HADOOP

WORKDIR /stackable
RUN curl -L https://repo.stackable.tech/repository/packages/spark/spark-${PRODUCT}-bin-hadoop${HADOOP}.tgz | tar -xzC .

RUN curl https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-0.16.1.jar \
    -o /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar && \
    chmod -x /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar

RUN mkdir /stackable/spark && \
    mv /stackable/spark-${PRODUCT}-bin-hadoop${HADOOP}/* /stackable/spark && \
    rm -rf /stackable/spark-${PRODUCT}-bin-hadoop${HADOOP}

WORKDIR /stackable/spark
CMD ["sbin/start-master.sh"]
