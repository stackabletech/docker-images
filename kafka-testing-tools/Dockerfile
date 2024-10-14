# syntax=docker/dockerfile:1.10.0@sha256:865e5dd094beca432e8c0a1d5e1c465db5f998dca4e439981029b3b81fb39ed5
# check=error=true

FROM stackable/image/kcat AS kcat

FROM stackable/image/stackable-base AS final

ARG PRODUCT
ARG KCAT
ARG RELEASE

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
COPY --chown=stackable:stackable --from=kcat /stackable/kcat-${KCAT}/kcat /stackable/kcat-${KCAT}
RUN ln -s /stackable/kcat-${KCAT} /stackable/kcat
COPY --chown=stackable:stackable --from=kcat /licenses /licenses


COPY --chown=stackable:stackable kafka-testing-tools/licenses /licenses

ENTRYPOINT ["/stackable/kcat"]
