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

RUN chown -R stackable:stackable /stackable

USER stackable


ARG PRODUCT_VERSION

RUN curl -L https://repo.stackable.tech/repository/packages/nifi/nifi-${PRODUCT_VERSION}-bin.tar.gz | tar -xzC .

WORKDIR /stackable/nifi-${PRODUCT_VERSION}
CMD ["bin/nifi.sh", "run"]
