# syntax=docker/dockerfile:1.6.0@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021

# Ignoring DL3038 globally because set `assumeyes=True` in dnf.conf in our base image
# Ignoring DL4006 globally because we inherit the SHELL from our base image
# hadolint global ignore=DL3038,DL4006

FROM stackable/image/java-base AS builder

ARG KCAT

RUN microdnf install \
    cmake \
    cyrus-sasl-devel \
    gcc-c++ \
    krb5-devel \
    libcurl-devel \
    make \
    openssl-devel \
    tar \
    wget \
    which \
    zlib \
    zlib-devel && \
    microdnf clean all

WORKDIR /stackable

RUN curl --fail -L -O https://repo.stackable.tech/repository/packages/kcat/kcat-${KCAT}.tar.gz \
    && tar xvfz kcat-${KCAT}.tar.gz \
    && cd kcat-${KCAT} \
    && ./bootstrap.sh

FROM stackable/image/stackable-base

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
RUN microdnf install -y zlib openssl cyrus-sasl libcurl diffutils binutils && \
    microdnf clean all

# Store kcat version with binary name and add softlink
COPY --chown=stackable:stackable --from=builder /stackable/kcat-${KCAT}/kcat /stackable/kcat-${KCAT}
RUN ln -s /stackable/kcat-${KCAT} /stackable/kcat
COPY --chown=stackable:stackable --from=builder /stackable/kcat-${KCAT}/LICENSE /licenses/KCAT_LICENSE


COPY --chown=stackable:stackable kafka-testing-tools/licenses /licenses

ENTRYPOINT ["/stackable/kcat"]
