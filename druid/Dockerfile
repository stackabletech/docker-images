# druid < 0.23 druid only fully supports java 8
# druid >= 0.23 is java 11 ready
FROM docker.stackable.tech/stackable/java-base:11-stackable0.2.2@sha256:7929833412c331fc23cde0e23ca730d652c0be61a8a69c8a82b2af937a3fbd4e

ARG PRODUCT
ARG AUTHORIZER
ARG RELEASE="1"

LABEL name="Apache Druid" \
      maintainer="info@stackable.de" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache Druid." \
      description="This image is deployed by the Stackable Operator for Apache Druid."

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf install findutils && \
    microdnf install tar gzip zip && \
    microdnf install shadow-utils && \
    microdnf clean all

COPY druid/stackable /stackable
COPY druid/licenses /licenses

RUN groupadd -r stackable --gid=1000 && \
    useradd -r -g stackable --uid=1000 stackable && \
    chown -R stackable:stackable /stackable

USER stackable
WORKDIR /stackable

RUN curl -L https://repo.stackable.tech/repository/packages/druid/apache-druid-${PRODUCT}-bin.tar.gz | tar -xzC . && \
    ln -s /stackable/apache-druid-${PRODUCT} /stackable/druid && \
    # Force to overwrite the existing 'run-druid'
    ln -sf /stackable/bin/run-druid /stackable/druid/bin/run-druid && \
    # Install the Prometheus emitter extension. This bundle contains the emitter and all jar dependencies.
    curl https://repo.stackable.tech/repository/packages/druid/druid-prometheus-emitter-${PRODUCT}.tar.gz | tar -xzC /stackable/druid/extensions && \
    # Install OPA authorizer extension.
    curl https://repo.stackable.tech/repository/packages/druid/druid-opa-authorizer-${AUTHORIZER}.tar.gz | tar -xzC /stackable/druid/extensions

WORKDIR /stackable/druid
CMD ["bin/run-druid", "coordinator", "conf/druid/cluster/master/coordinator-overlord/"]
