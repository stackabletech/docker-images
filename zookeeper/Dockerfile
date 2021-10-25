FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

# PRODUCT_VERSION must be at least 3.5.0 in order to work
ARG PRODUCT_VERSION
ARG STACKABLE_APP_SHORT_NAME=zookeeper-$PRODUCT_VERSION
ARG STACKABLE_APP_FULL_NAME=apache-zookeeper-$PRODUCT_VERSION-bin
ARG VOLUME_PATH=/stackable/zookeeper

# Required directories
ARG CONF_DIR=/conf
ARG DATA_DIR=/data

# Create required directories
RUN mkdir -p "$VOLUME_PATH$CONF_DIR" "$VOLUME_PATH$DATA_DIR"

# Download java, tar and gzip
RUN microdnf update && \
    microdnf install java-11-openjdk-headless tar gzip && \
    microdnf clean all

ENV JAVA_HOME=/usr/lib/jvm/jre-11

# Download ZooKeeper
WORKDIR /opt/stackable/packages
RUN curl -L https://archive.apache.org/dist/zookeeper/$STACKABLE_APP_SHORT_NAME/$STACKABLE_APP_FULL_NAME.tar.gz | tar -xzC .

# Copy stackable folder (jmx exporter and config)
ADD stackable /opt/stackable/packages/$STACKABLE_APP_FULL_NAME/stackable

WORKDIR /opt/stackable/packages/$STACKABLE_APP_FULL_NAME/stackable/lib
# Download JMX prometheus exporter to stackable/lib
RUN curl https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-0.16.1.jar \
    -o /opt/stackable/packages/$STACKABLE_APP_FULL_NAME/stackable/lib/jmx_prometheus_javaagent-0.16.1.jar

WORKDIR /opt/stackable/packages/$STACKABLE_APP_FULL_NAME
ENTRYPOINT ["bin/zkServer.sh", "start-foreground", "/stackable/zookeeper/conf/zoo.cfg"]