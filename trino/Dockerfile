FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

# Download java, tar and gzip
RUN microdnf update && \
    microdnf install java-11-openjdk-headless --nodocs && \
    microdnf install tar gzip python3 && \
    microdnf install shadow-utils && \
    microdnf clean all

ENV JAVA_HOME=/usr/lib/jvm/jre-11

RUN groupadd -r stackable --gid=1000; \
    useradd -r -g stackable --uid=1000 stackable;

ADD stackable /stackable
RUN chown -R stackable:stackable /stackable

RUN alternatives --set python /usr/bin/python3

USER stackable

ARG PRODUCT

WORKDIR /stackable
RUN curl -L https://repo.stackable.tech/repository/packages/trino-server/trino-server-${PRODUCT}.tar.gz | tar -xzC .

RUN curl https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-0.16.1.jar \
    -o /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar && \
    chmod -x /stackable/jmx/jmx_prometheus_javaagent-0.16.1.jar

WORKDIR /stackable/trino-server-${PRODUCT}/plugin
RUN curl -L https://repo.stackable.tech/repository/packages/trino-opa-authorizer/trino-opa-authorizer-0.1.0.tar.gz | tar -xzC .

RUN mkdir /stackable/trino-server && \
    mv /stackable/trino-server-${PRODUCT}/* /stackable/trino-server && \
    rm -rf /stackable/trino-server-${PRODUCT}

WORKDIR /stackable/trino-server
CMD ["bin/launcher", "run", "--etc-dir=/stackable/conf"]
