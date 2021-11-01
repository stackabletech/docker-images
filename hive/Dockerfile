FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

# Download java, tar and gzip
RUN microdnf update \
    && microdnf install --nodocs java-11-openjdk-headless tar gzip which shadow-utils \
    && microdnf clean all

ENV JAVA_HOME=/usr/lib/jvm/jre-11

# Add user & group
RUN groupadd stackable && \
    useradd stackable -g stackable 

WORKDIR /stackable
ADD stackable .
RUN chown -R stackable:stackable /stackable

USER stackable

ARG PRODUCT_VERSION
ARG HADOOP_VERSION=2.10.1

RUN curl -L https://repo.stackable.tech/repository/packages/hive/apache-hive-${PRODUCT_VERSION}-bin.tar.gz | tar -xzC .

RUN curl https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-0.16.1.jar \
    -o /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar && \
    chmod -x /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar

WORKDIR /stackable/apache-hive-${PRODUCT_VERSION}-bin/apache-hive-${PRODUCT_VERSION}-bin
ENV HADOOP_HOME=/stackable/apache-hive-${PRODUCT_VERSION}-bin/hadoop-${HADOOP_VERSION}
COPY /stackable/bin/start-metastore bin
CMD ["./bin/start-metastore", "--config", "conf", "--hive-bin-dir", "bin", "--db-type", "derby"]
