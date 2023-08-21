# syntax=docker/dockerfile:1
FROM stackable/image/java-base

ARG PRODUCT
ARG RELEASE

LABEL name="EDC" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for the EDC." \
      description="This image is deployed by the Stackable Operator for the EDC."

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf clean all

USER stackable
WORKDIR /stackable

RUN curl -L https://repo.stackable.tech/repository/packages/edc/dataspace-connector-${PRODUCT}.jar -o connector.jar

CMD ["java", "-Dedc.fs.config=./mount/config/config.properties", "-jar", "connector.jar" ]
