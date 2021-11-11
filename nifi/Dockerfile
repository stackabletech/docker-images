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

WORKDIR /stackable
RUN curl -L https://repo.stackable.tech/repository/packages/nifi/nifi-${PRODUCT_VERSION}-bin.tar.gz | tar -xzC .

RUN mkdir /stackable/nifi && \
    mv /stackable/nifi-${PRODUCT_VERSION}/* /stackable/nifi && \
    rm -rf /stackable/nifi-${PRODUCT_VERSION}

WORKDIR /stackable/nifi
CMD ["bin/nifi.sh", "run"]
