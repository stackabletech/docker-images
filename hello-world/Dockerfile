# syntax=docker/dockerfile:1.8.1@sha256:e87caa74dcb7d46cd820352bfea12591f3dba3ddc4285e19c7dcd13359f7cefd

FROM stackable/image/java-base

ARG PRODUCT
ARG RELEASE

LABEL name="Hello World" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for the Stackable Hello World Operator" \
      description="This image is deployed by the Stackable Hello World Operator."

RUN microdnf update && \
    microdnf clean all && \
    rm -rf /var/cache/yum

USER stackable
WORKDIR /stackable

RUN curl --fail -L https://repo.stackable.tech/repository/packages/hello-world/hello-world-${PRODUCT}.jar -o hello-world.jar

CMD ["java", "-jar", "hello-world.jar"]
