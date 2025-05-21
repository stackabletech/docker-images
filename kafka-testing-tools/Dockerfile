# syntax=docker/dockerfile:1.15.1@sha256:9857836c9ee4268391bb5b09f9f157f3c91bb15821bb77969642813b0d00518d
# check=error=true

FROM stackable/image/kafka/kcat AS kcat

FROM stackable/image/stackable-base AS final

ARG PRODUCT
ARG KAFKA_KCAT
ARG RELEASE
ARG STACKABLE_USER_UID

LABEL name="Kafka Testing Tools" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for the kcat tool." \
      description="Used for integration testing"

 # diffutils (diff) and binutils (cmp) are needed for the kafka integration tests
RUN microdnf install  \
    binutils \
    cyrus-sasl \
    diffutils \
    libcurl-minimal \
    openssl \
    zlib \
    && microdnf clean all \
    && rm -rf /var/cache/yum

# Store kcat version with binary name and add softlink
COPY --chown=${STACKABLE_USER_UID}:0 --from=kcat /stackable/kcat-${KAFKA_KCAT}/kcat /stackable/kcat-${KAFKA_KCAT}
RUN ln -s /stackable/kcat-${KAFKA_KCAT} /stackable/kcat
COPY --chown=${STACKABLE_USER_UID}:0 --from=kcat /licenses /licenses

COPY --chown=${STACKABLE_USER_UID}:0 kafka-testing-tools/licenses /licenses

ENTRYPOINT ["/stackable/kcat"]
