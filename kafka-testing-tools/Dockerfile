# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
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
COPY --chown=${STACKABLE_USER_UID}:0 --from=kcat /stackable/kcat /stackable/kcat-${KAFKA_KCAT}
COPY --chown=${STACKABLE_USER_UID}:0 --from=kcat /stackable/kcat-${KAFKA_KCAT}-src.tar.gz /stackable
RUN ln -s /stackable/kcat-${KAFKA_KCAT} /stackable/kcat
COPY --chown=${STACKABLE_USER_UID}:0 --from=kcat /licenses /licenses

COPY --chown=${STACKABLE_USER_UID}:0 kafka-testing-tools/licenses /licenses

ENTRYPOINT ["/stackable/kcat"]
