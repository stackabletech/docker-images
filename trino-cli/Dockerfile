# syntax=docker/dockerfile:1@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021
FROM stackable/image/java-base

ARG PRODUCT

LABEL name="Trino CLI" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for the Trino CLI." \
      description="This image can be used to interact with Stackable Trino deployments."

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf install tar gzip zip && \
    microdnf clean all

USER stackable
WORKDIR /stackable

COPY --chown=stackable:stackable trino-cli/licenses /licenses

WORKDIR /stackable/trino-cli

RUN curl --fail -L -O https://repo.stackable.tech/repository/packages/trino-cli/trino-cli-${PRODUCT}-executable.jar \
    && ln -s trino-cli-${PRODUCT}-executable.jar trino-cli-executable.jar

ENTRYPOINT ["java", "-jar", "/stackable/trino-cli/trino-cli-executable.jar"]
