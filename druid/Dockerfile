FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

# Download java, tar and gzip
RUN microdnf update && \
    # druid only fully supports java 8 for now \
    # https://druid.apache.org/docs/latest/tutorials/index.html#requirements
    microdnf install java-1.8.0-openjdk --nodocs && \
    microdnf install findutils && \
    microdnf install tar gzip && \
    microdnf install shadow-utils && \
    microdnf clean all

ENV JAVA_HOME /etc/alternatives/jre

# Add user & group
RUN groupadd stackable && \
    useradd stackable -g stackable

ARG PRODUCT_VERSION

WORKDIR /stackable
RUN curl -L https://repo.stackable.tech/repository/packages/druid/apache-druid-${PRODUCT_VERSION}-bin.tar.gz | tar -xzC .

WORKDIR /stackable/apache-druid-${PRODUCT_VERSION}
ADD stackable/script/run-druid ./stackable/run-druid

RUN chmod +x stackable/run-druid && \
    chown -R stackable:stackable /stackable

USER stackable

CMD ["./stackable/run-druid", "coordinator"]
