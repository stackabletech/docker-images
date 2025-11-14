# syntax=docker/dockerfile:1.20.0@sha256:26147acbda4f14c5add9946e2fd2ed543fc402884fd75146bd342a7f6271dc1d
# check=error=true

FROM local-image/kafka/kcat AS kcat

FROM local-image/stackable-base AS final

ARG PRODUCT_VERSION
ARG KAFKA_KCAT_VERSION
ARG RELEASE_VERSION
ARG STACKABLE_USER_UID

LABEL name="Kafka Testing Tools" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT_VERSION}" \
      release="${RELEASE_VERSION}" \
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
COPY --chown=${STACKABLE_USER_UID}:0 --from=kcat /stackable/kcat /stackable/kcat-${KAFKA_KCAT_VERSION}
COPY --chown=${STACKABLE_USER_UID}:0 --from=kcat /stackable/kcat-${KAFKA_KCAT_VERSION}-src.tar.gz /stackable
RUN ln -s /stackable/kcat-${KAFKA_KCAT_VERSION} /stackable/kcat
COPY --chown=${STACKABLE_USER_UID}:0 --from=kcat /licenses /licenses

COPY --chown=${STACKABLE_USER_UID}:0 kafka-testing-tools/licenses /licenses

ENTRYPOINT ["/stackable/kcat"]
