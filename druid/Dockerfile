# syntax=docker/dockerfile:1@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021
# druid < 0.23 druid only fully supports java 8
# druid >= 0.23 is java 11 ready
FROM stackable/image/java-base

ARG PRODUCT
ARG JACKSON_DATAFORMAT_XML
ARG STAX2_API
ARG WOODSTOX_CORE
ARG AUTHORIZER
ARG RELEASE

LABEL name="Apache Druid" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache Druid." \
      description="This image is deployed by the Stackable Operator for Apache Druid."

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf install findutils tar gzip zip && \
    microdnf clean all

USER stackable
WORKDIR /stackable

COPY --chown=stackable:stackable druid/stackable /stackable
COPY --chown=stackable:stackable druid/licenses /licenses

RUN curl --fail -L https://repo.stackable.tech/repository/packages/druid/apache-druid-${PRODUCT}-bin.tar.gz | tar -xzC . && \
    ln -s /stackable/apache-druid-${PRODUCT} /stackable/druid && \
    # Force to overwrite the existing 'run-druid'
    ln -sf /stackable/bin/run-druid /stackable/druid/bin/run-druid && \
    # Install the Prometheus emitter extension. This bundle contains the emitter and all jar dependencies.
    curl --fail https://repo.stackable.tech/repository/packages/druid/druid-prometheus-emitter-${PRODUCT}.tar.gz | tar -xzC /stackable/druid/extensions && \
    # Install OPA authorizer extension.
    curl --fail https://repo.stackable.tech/repository/packages/druid/druid-opa-authorizer-${AUTHORIZER}.tar.gz | tar -xzC /stackable/druid/extensions && \
    # Install jackson-dataformat-xml, stax2-api, and woodstox-core which are required for logging, and remove stax-ex.
    rm /stackable/druid/lib/stax-ex-*.jar && \
    curl --fail -L -o /stackable/druid/lib/jackson-dataformat-xml-${JACKSON_DATAFORMAT_XML}.jar \
        https://repo.stackable.tech/repository/packages/jackson-dataformat-xml/jackson-dataformat-xml-${JACKSON_DATAFORMAT_XML}.jar && \
    curl --fail -L -o /stackable/druid/lib/stax2-api-${STAX2_API}.jar \
        https://repo.stackable.tech/repository/packages/stax2-api/stax2-api-${STAX2_API}.jar && \
    curl --fail -L -o /stackable/druid/lib/woodstox-core-${WOODSTOX_CORE}.jar \
        https://repo.stackable.tech/repository/packages/woodstox-core/woodstox-core-${WOODSTOX_CORE}.jar

WORKDIR /stackable/druid
CMD ["bin/run-druid", "coordinator", "conf/druid/cluster/master/coordinator-overlord/"]
