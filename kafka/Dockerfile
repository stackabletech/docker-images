FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

RUN microdnf update && \
    microdnf install java-11-openjdk-headless --nodocs && \
    microdnf install tar gzip && \
    microdnf install shadow-utils && \
    microdnf clean all

ENV JAVA_HOME=/usr/lib/jvm/jre-11

RUN groupadd -r stackable --gid=1000; \
    useradd -r -g stackable --uid=1000 stackable;

ADD stackable /stackable
RUN chown -R stackable:stackable /stackable

USER stackable

ARG PRODUCT_VERSION
ARG SCALA_VERSION
ARG OPA_AUTHORIZER_VERSION

WORKDIR /stackable
RUN curl -L https://repo.stackable.tech/repository/packages/kafka/kafka_${SCALA_VERSION}-${PRODUCT_VERSION}.tgz | tar -xzC .

RUN curl -L https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-0.16.1.jar \
    -o /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar

RUN mkdir /stackable/kafka && \
    mv /stackable/kafka_${SCALA_VERSION}-${PRODUCT_VERSION}/* /stackable/kafka && \
    rm -rf /stackable/kafka_${SCALA_VERSION}-${PRODUCT_VERSION}

RUN curl -L https://repo.stackable.tech/repository/packages/kafka-opa-authorizer/opa-authorizer-${OPA_AUTHORIZER_VERSION}-all.jar \
    -o /stackable/kafka/libs/opa-authorizer-${OPA_AUTHORIZER_VERSION}-all.jar

WORKDIR /stackable/kafka
CMD ["bin/kafka-server-start.sh", "/stackable/conf/server.properties"]
