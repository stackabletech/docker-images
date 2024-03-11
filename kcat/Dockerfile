# syntax=docker/dockerfile:1@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021
FROM stackable/image/java-base AS builder

ARG PRODUCT

 # diff and cmp are needed for the kafka integration tests
RUN microdnf install -y zlib-devel openssl-devel cyrus-sasl-devel libcurl-devel && \
    microdnf install -y tar which wget zlib gcc-c++ make cmake && \
    microdnf clean all

WORKDIR /stackable

RUN curl --fail -L -O https://repo.stackable.tech/repository/packages/kcat/kcat-${PRODUCT}.tar.gz \
    && tar xvfz kcat-${PRODUCT}.tar.gz \
    && cd kcat-${PRODUCT} \
    && ./bootstrap.sh

FROM stackable/image/stackable-base

ARG PRODUCT

LABEL name="kcat" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for the kcat tool." \
      description="Used for integration testing"

 # diffutils (diff) and binutils (cmp) are needed for the kafka integration tests
RUN microdnf install -y zlib openssl cyrus-sasl libcurl diffutils binutils && \
    microdnf clean all


# Store kcat version with binary name and add softlink
COPY --chown=stackable:stackable --from=builder /stackable/kcat-${PRODUCT}/kcat /stackable/kcat-${PRODUCT}
RUN ln -s /stackable/kcat-${PRODUCT} /stackable/kcat
COPY --chown=stackable:stackable --from=builder /stackable/kcat-${PRODUCT}/LICENSE /licenses/KCAT_LICENSE


COPY --chown=stackable:stackable kcat/licenses /licenses

ENTRYPOINT ["/stackable/kcat"]
