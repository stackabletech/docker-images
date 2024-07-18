# syntax=docker/dockerfile:1.8.1@sha256:e87caa74dcb7d46cd820352bfea12591f3dba3ddc4285e19c7dcd13359f7cefd

FROM stackable/image/kcat AS kcat

FROM stackable/image/stackable-base AS final

ARG PRODUCT
ARG KCAT

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
