FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    # druid only fully supports java 8 for now
    # https://druid.apache.org/docs/latest/tutorials/index.html#requirements
    microdnf install java-1.8.0-openjdk --nodocs && \
    microdnf install findutils && \
    microdnf install tar gzip zip && \
    microdnf install shadow-utils && \
    microdnf clean all

ENV JAVA_HOME /etc/alternatives/jre

COPY druid/stackable /stackable
RUN groupadd -r stackable --gid=1000 && \
    useradd -r -g stackable --uid=1000 stackable && \
    chown -R stackable:stackable /stackable

USER stackable

ARG PRODUCT

WORKDIR /stackable
RUN curl -L https://repo.stackable.tech/repository/packages/druid/apache-druid-${PRODUCT}-bin.tar.gz | tar -xzC . && \
    ln -s /stackable/apache-druid-${PRODUCT} /stackable/druid && \
    # Force to overwrite the existing 'run-druid'
    ln -sf /stackable/bin/run-druid /stackable/druid/bin/run-druid && \
    # Install the Prometheus emitter extension. This bundle contains the emitter and all jar dependencies.
    curl https://repo.stackable.tech/repository/packages/druid/druid-prometheus-emitter-${PRODUCT}.tar.gz | tar -xzC /stackable/druid/extensions && \
    # Install OPA authorizer extension.
    curl https://repo.stackable.tech/repository/packages/druid/druid-opa-authorizer-${PRODUCT}.tar.gz | tar -xzC /stackable/druid/extensions

WORKDIR /stackable/druid
CMD ["bin/run-druid", "coordinator", "conf/druid/cluster/master/coordinator-overlord/"]
