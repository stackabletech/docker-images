# syntax=docker/dockerfile:1

# TODO: Do we even support druid < 0.23 anymore?
# druid < 0.23 druid only fully supports java 8
# druid >= 0.23 is java 11 ready
FROM stackable/image/java-base AS builder

ARG PRODUCT
ARG JACKSON_DATAFORMAT_XML
ARG STAX2_API
ARG WOODSTOX_CORE
ARG AUTHORIZER

RUN microdnf update && \
    microdnf install findutils openssl tar gzip zip && \
    microdnf clean all && \
    rm -rf /var/cache/yum

WORKDIR /stackable

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

# Download & extracd Apache Druid itself
RUN curl --fail -L https://repo.stackable.tech/repository/packages/druid/apache-druid-${PRODUCT}-bin.tar.gz | tar -xzC . && \
    ln -s /stackable/apache-druid-${PRODUCT} /stackable/druid

# Overwrite the existing 'run-druid'
# TODO: Why? Where does this come from?
COPY --chown=stackable:stackable druid/stackable /stackable
RUN ln -sf /stackable/bin/run-druid /stackable/druid/bin/run-druid

# Install the Prometheus emitter extension. This bundle contains the emitter and all jar dependencies.
RUN curl --fail -L https://repo.stackable.tech/repository/packages/druid/druid-prometheus-emitter-${PRODUCT}.tar.gz | tar -xzC /stackable/druid/extensions

# Install OPA authorizer extension.
RUN curl --fail -L https://repo.stackable.tech/repository/packages/druid/druid-opa-authorizer-${AUTHORIZER}.tar.gz | tar -xzC /stackable/druid/extensions

# Install jackson-dataformat-xml, stax2-api, and woodstox-core which are required for logging, and remove stax-ex.
# TODO: Why remove stax-ex?
RUN rm /stackable/druid/lib/stax-ex-*.jar && \
    curl --fail -L -o /stackable/druid/lib/jackson-dataformat-xml-${JACKSON_DATAFORMAT_XML}.jar \
        https://repo.stackable.tech/repository/packages/jackson-dataformat-xml/jackson-dataformat-xml-${JACKSON_DATAFORMAT_XML}.jar && \
    curl --fail -L -o /stackable/druid/lib/stax2-api-${STAX2_API}.jar \
        https://repo.stackable.tech/repository/packages/stax2-api/stax2-api-${STAX2_API}.jar && \
    curl --fail -L -o /stackable/druid/lib/woodstox-core-${WOODSTOX_CORE}.jar \
        https://repo.stackable.tech/repository/packages/woodstox-core/woodstox-core-${WOODSTOX_CORE}.jar


# Final Image
FROM stackable/image/java-base

ARG PRODUCT
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

# TODO: Not sure what openssl is required for
RUN microdnf update && \
    microdnf install openssl && \
    microdnf clean all && \
    rm -rf /var/cache/yum

USER stackable
WORKDIR /stackable

COPY --chown=stackable:stackable druid/licenses /licenses

ENV HOME=/stackable
ENV PATH=$PATH:$HOME/druid/binp

COPY --link --chown=stackable:stackable --from=builder /stackable/apache-druid-${PRODUCT} /stackable/apache-druid-${PRODUCT}/
RUN ln -s /stackable/apache-druid-${PRODUCT} /stackable/druid

WORKDIR /stackable/druid
CMD ["bin/run-druid", "coordinator", "conf/druid/cluster/master/coordinator-overlord/"]
