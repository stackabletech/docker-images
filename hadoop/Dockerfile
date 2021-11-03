FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

ARG PRODUCT_VERSION

# Download java, tar and gzip
RUN microdnf update && \
    microdnf install java-11-openjdk-headless --nodocs && \
    microdnf install tar gzip && \
    microdnf install shadow-utils && \
    microdnf clean all

ENV JAVA_HOME=/usr/lib/jvm/jre-11
ENV HDFS_BIN_DIR=/stackable/hadoop-${PRODUCT_VERSION}/bin

# Add user & group
RUN groupadd stackable && \
    useradd stackable -g stackable 

WORKDIR /stackable
ADD stackable .

RUN curl -L https://repo.stackable.tech/repository/packages/hadoop/hadoop-${PRODUCT_VERSION}.tar.gz | tar -xzC .

RUN curl https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-0.16.1.jar \
    -o /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar && \
    chmod -x /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar

WORKDIR /stackable/hadoop-${PRODUCT_VERSION}
COPY /stackable/bin/start-namenode bin

RUN chown -R stackable:stackable /stackable
USER stackable

CMD ["./bin/start-namenode", "--config", "etc/hadoop"]
