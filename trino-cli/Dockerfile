# syntax=docker/dockerfile:1.10.0@sha256:865e5dd094beca432e8c0a1d5e1c465db5f998dca4e439981029b3b81fb39ed5
# check=error=true

FROM stackable/image/java-base

ARG PRODUCT
ARG RELEASE

LABEL name="Trino CLI" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for the Trino CLI." \
      description="This image can be used to interact with Stackable Trino deployments."

RUN microdnf update && \
    microdnf install \
    gzip \
    tar \
    zip && \
    microdnf clean all && \
    rm -rf /var/cache/yum

USER stackable
WORKDIR /stackable

COPY --chown=stackable:stackable trino-cli/licenses /licenses

WORKDIR /stackable/trino-cli

RUN curl -O https://repo.stackable.tech/repository/packages/trino-cli/trino-cli-${PRODUCT}-executable.jar \
    && ln -s trino-cli-${PRODUCT}-executable.jar trino-cli-executable.jar

ENTRYPOINT ["java", "-jar", "/stackable/trino-cli/trino-cli-executable.jar"]
