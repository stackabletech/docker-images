# syntax=docker/dockerfile:1
# druid < 0.23 druid only fully supports java 8
# druid >= 0.23 is java 11 ready
FROM stackable/image/java-base

ARG PRODUCT
ARG AUTHORIZER
ARG RELEASE

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
    microdnf install findutils openssl tar gzip zip shadow-utils && \
    microdnf clean all

USER stackable
WORKDIR /stackable

COPY --chown=stackable:stackable druid/stackable /stackable
COPY --chown=stackable:stackable druid/licenses /licenses

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
