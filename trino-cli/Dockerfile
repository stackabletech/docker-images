# syntax=docker/dockerfile:1.6.0@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021

# Ignoring DL3038 globally because set `assumeyes=True` in dnf.conf in our base image
# Ignoring DL4006 globally because we inherit the SHELL from our base image
# hadolint global ignore=DL3038,DL4006

FROM stackable/image/java-base

ARG PRODUCT

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

RUN curl --fail -L -O https://repo.stackable.tech/repository/packages/trino-cli/trino-cli-${PRODUCT}-executable.jar \
    && ln -s trino-cli-${PRODUCT}-executable.jar trino-cli-executable.jar

ENTRYPOINT ["java", "-jar", "/stackable/trino-cli/trino-cli-executable.jar"]
