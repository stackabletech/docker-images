# syntax=docker/dockerfile:1
FROM stackable/image/java-base

ARG PRODUCT
ARG RELEASE

LABEL name="Hello World" \
      maintainer="info@stackable.de" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache Druid." \
      description="This image is deployed by the Stackable Operator for Apache Druid."

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf clean all

USER stackable
WORKDIR /stackable

RUN curl -L https://repo.stackable.tech/repository/packages/edc/dataspace-connector-${PRODUCT}.jar -o connector.jar

CMD ["java", "-Dedc.fs.config=./mount/config/config.properties", "-jar", "connector.jar" ]
