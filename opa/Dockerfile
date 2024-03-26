# syntax=docker/dockerfile:1.6.0@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021

# Ignoring DL4006 globally because we inherit the SHELL from our base image
# hadolint global ignore=DL4006

FROM stackable/image/vector AS opa-bundle-builder

ARG BUNDLE_BUILDER_VERSION

# Update image and install everything needed for Rustup & Rust
RUN microdnf update --disablerepo=* --enablerepo=ubi-8-appstream-rpms --enablerepo=ubi-8-baseos-rpms -y \
  && rm -rf /var/cache/yum \
  && microdnf install --disablerepo=* --enablerepo=ubi-8-appstream-rpms --enablerepo=ubi-8-baseos-rpms curl findutils gcc gcc-c++ git make cmake openssl-devel pkg-config systemd-devel unzip -y \
  && rm -rf /var/cache/yum

WORKDIR /

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN git clone --depth 1 --branch ${BUNDLE_BUILDER_VERSION} https://github.com/stackabletech/opa-bundle-builder
RUN cd ./opa-bundle-builder && . $HOME/.cargo/env && cargo build --release

FROM stackable/image/vector AS multilog-builder

ARG DAEMONTOOLS_VERSION=0.76

COPY opa/daemontools /daemontools

RUN microdnf update -y && \
    microdnf install tar gzip yum make gcc -y && \
    yum install patch -y && \
    yum clean all -y && \
    microdnf clean all

WORKDIR /daemontools

RUN tar xzf daemontools-${DAEMONTOOLS_VERSION}.tar.gz

WORKDIR /daemontools/admin/daemontools-${DAEMONTOOLS_VERSION}/src

RUN patch < /daemontools/conf-cc.patch && \
    patch multilog.c < /daemontools/multilog_max_file_size.patch

WORKDIR /daemontools/admin/daemontools-${DAEMONTOOLS_VERSION}

RUN package/install

FROM stackable/image/vector

ARG PRODUCT
ARG RELEASE
ARG TARGETARCH
ARG TARGETOS

LABEL name="Open Policy Agent" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for OPA." \
      description="This image is deployed by the Stackable Operator for OPA."

RUN microdnf update && \
    microdnf install tar gzip && \
    microdnf clean all

COPY opa/licenses /licenses

USER stackable
WORKDIR /stackable/opa

RUN curl --fail -L "https://repo.stackable.tech/repository/packages/opa/opa_${TARGETOS}_${TARGETARCH}_static_${PRODUCT}" \
     -o opa_${TARGETOS}_${TARGETARCH}_static_${PRODUCT} && \
    chmod +x opa_${TARGETOS}_${TARGETARCH}_static_${PRODUCT} && \
    ln -s opa_${TARGETOS}_${TARGETARCH}_static_${PRODUCT} opa

COPY --from=opa-bundle-builder --chown=stackable:stackable /opa-bundle-builder/target/release/stackable-opa-bundle-builder /stackable/opa-bundle-builder
COPY --from=multilog-builder --chown=stackable:stackable /daemontools/admin/daemontools/command/multilog /stackable/multilog

CMD ["./opa", "run", "-s"]
