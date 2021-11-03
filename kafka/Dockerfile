FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

ARG PRODUCT_VERSION
ARG SCALA_VERSION
ARG OPA_AUTHORIZER_VERSION=1.1.0

# Download java, tar and gzip
RUN microdnf update && \
    microdnf install java-11-openjdk-headless --nodocs && \
    microdnf install tar gzip && \
    microdnf install shadow-utils && \
    microdnf clean all

ENV JAVA_HOME=/usr/lib/jvm/jre-11

# Add user & group
RUN groupadd stackable && \
    useradd stackable -g stackable 

WORKDIR /stackable
ADD stackable .

# kafka image
RUN curl -L https://archive.apache.org/dist/kafka/${PRODUCT_VERSION}/kafka_${SCALA_VERSION}-${PRODUCT_VERSION}.tgz| tar -xzC .

# jmx exporter
RUN curl -L https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-0.16.1.jar \
    -o /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar

WORKDIR /stackable/kafka_${SCALA_VERSION}-${PRODUCT_VERSION}

# opa authorizer
RUN curl -L https://github.com/Bisnode/opa-kafka-plugin/releases/download/v${OPA_AUTHORIZER_VERSION}/opa-authorizer-${OPA_AUTHORIZER_VERSION}-all.jar \
    -o libs/opa-authorizer-${OPA_AUTHORIZER_VERSION}-all.jar

RUN chown -R stackable:stackable /stackable
USER stackable

CMD ["bin/kafka-server-start.sh", "/stackable/conf/server.properties"]
