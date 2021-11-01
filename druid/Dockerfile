FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

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

RUN chown -R stackable:stackable /stackable

USER stackable

ARG PRODUCT_VERSION
ARG DRUID_EXPORTER_VERSION

RUN curl -L https://repo.stackable.tech/repository/packages/druid/apache-druid-${PRODUCT_VERSION}-bin.tar.gz | tar -xzC .

RUN curl https://github.com/opstree/druid-exporter/releases/download/v0.11/druid-exporter-v${DRUID_EXPORTER_VERSION}-linux-amd64.tar.gz \
    -o /stackable/jmx/druid-exporter && \
    chmod -x /stackable/jmx/druid-exporter

VOLUME /stackable/data/
WORKDIR /stackable/apache-druid-${PRODUCT_VERSION}

CMD ["bin/coordinator.sh", "start"]
