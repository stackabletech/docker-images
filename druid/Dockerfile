FROM registry.access.redhat.com/ubi8/ubi-minimal AS builder
LABEL maintainer="Stackable GmbH"

RUN microdnf update && \
    # druid only fully supports java 8 for now \
    # https://druid.apache.org/docs/latest/tutorials/index.html#requirements
    microdnf install java-1.8.0-openjdk --nodocs && \
    microdnf install findutils && \
    microdnf install tar gzip && \
    microdnf install shadow-utils && \
    microdnf clean all

ENV JAVA_HOME /etc/alternatives/jre

RUN groupadd -r stackable --gid=1000; \
    useradd -r -g stackable --uid=1000 stackable;

ADD stackable /stackable
RUN chown -R stackable:stackable /stackable

USER stackable

ARG PRODUCT

WORKDIR /stackable
RUN curl -L https://repo.stackable.tech/repository/packages/druid/apache-druid-${PRODUCT}-bin.tar.gz | tar -xzC .

RUN mkdir /stackable/druid && \
    mv /stackable/apache-druid-${PRODUCT}/* /stackable/druid && \
    rm -rf /stackable/apache-druid-${PRODUCT} && \
    mv /stackable/bin/run-druid /stackable/druid/bin

# Install the Prometheus emitter extension. This bundle contains the emitter and all jar dependencies.
RUN curl https://repo.stackable.tech/repository/packages/druid/druid-prometheus-emitter-${PRODUCT}.tar.gz | tar -xzC /stackable/druid/extensions

WORKDIR /stackable/druid
CMD ["bin/run-druid", "coordinator", "conf/druid/cluster/master/coordinator-overlord/"]
