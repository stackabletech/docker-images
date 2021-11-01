FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

# ZooKeeper PRODUCT_VERSION must be at least 3.5.0 in order to work
ARG PRODUCT_VERSION
ARG STACKABLE_APP_SHORT_NAME=zookeeper-${PRODUCT_VERSION}
ARG STACKABLE_APP_FULL_NAME=apache-zookeeper-${PRODUCT_VERSION}-bin

# Download java, tar and gzip
RUN microdnf update && \
    microdnf install java-11-openjdk-headless --nodocs && \
    microdnf install tar gzip && \
    microdnf install shadow-utils && \
    microdnf clean all

ENV JAVA_HOME=/usr/lib/jvm/jre-11

# Download ZooKeeper
WORKDIR /stackable
RUN curl -L https://archive.apache.org/dist/zookeeper/${STACKABLE_APP_SHORT_NAME}/${STACKABLE_APP_FULL_NAME}.tar.gz | tar -xzC .

# Copy stackable folder (jmx exporter and config, myid script for init_container)
ADD stackable .

# Download JMX prometheus exporter to stackable/lib
RUN curl https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-0.16.1.jar \
    -o /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar && \
    chmod -x /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar

# Add user & group
RUN groupadd stackable && \
    useradd stackable -g stackable && \
    chown -R stackable /stackable

USER stackable

WORKDIR /stackable/${STACKABLE_APP_FULL_NAME}
CMD ["bin/zkServer.sh", "start-foreground", "/stackable/conf/zoo.cfg"]
