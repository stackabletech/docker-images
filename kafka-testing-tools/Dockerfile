# syntax=docker/dockerfile:1.6.0@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021

# Ignoring DL3038 globally because set `assumeyes=True` in dnf.conf in our base image
# Ignoring DL4006 globally because we inherit the SHELL from our base image
# hadolint global ignore=DL3038,DL4006

# Not tagging base image because it is built as part of the same process
# hadolint ignore=DL3006
FROM stackable/image/kcat AS kcat

# Not tagging base image because it is built as part of the same process
# hadolint ignore=DL3006
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
