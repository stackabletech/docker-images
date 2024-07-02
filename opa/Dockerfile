# syntax=docker/dockerfile:1.8.1@sha256:e87caa74dcb7d46cd820352bfea12591f3dba3ddc4285e19c7dcd13359f7cefd

# Ignoring DL3038 globally because set `assumeyes=True` in dnf.conf in our base image
# Ignoring DL4006 globally because we inherit the SHELL from our base image
# hadolint global ignore=DL3038,DL4006

# Not tagging base image because it is built as part of the same process
# hadolint ignore=DL3006
FROM stackable/image/stackable-base AS opa-bundle-builder

ARG BUNDLE_BUILDER_VERSION

# Update image and install everything needed for Rustup & Rust
RUN microdnf update \
  && microdnf install \
    cmake \
    findutils \
    gcc \
    gcc-c++ \
    git \
    make \
    openssl-devel \
    pkg-config \
    systemd-devel \
    unzip \
  && rm -rf /var/cache/yum

WORKDIR /

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN git clone --depth 1 --branch ${BUNDLE_BUILDER_VERSION} https://github.com/stackabletech/opa-bundle-builder
RUN cd ./opa-bundle-builder && . $HOME/.cargo/env && cargo build --release

# Not tagging base image because it is built as part of the same process
# hadolint ignore=DL3006
FROM stackable/image/stackable-base AS multilog-builder

ARG DAEMONTOOLS_VERSION=0.76

COPY opa/daemontools /daemontools

RUN microdnf update && \
    microdnf install \
    gcc \
    gzip \
    make \
    patch \
    tar && \
    microdnf clean all \
    && rm -rf /var/cache/yum

WORKDIR /daemontools

RUN tar xzf daemontools-${DAEMONTOOLS_VERSION}.tar.gz

WORKDIR /daemontools/admin/daemontools-${DAEMONTOOLS_VERSION}/src

RUN patch < /daemontools/conf-cc.patch && \
    patch multilog.c < /daemontools/multilog_max_file_size.patch

WORKDIR /daemontools/admin/daemontools-${DAEMONTOOLS_VERSION}

RUN package/install

# Not tagging base image because it is built as part of the same process
# hadolint ignore=DL3006
FROM stackable/image/stackable-base as opa-builder

ARG PRODUCT
ARG RELEASE
ARG TARGETARCH
ARG TARGETOS

ENV GOARCH=$TARGETARCH
ENV GOOS=$TARGETOS

# go - used to build OPA
# gzip, tar - used to unpack the OPA source
# We don't pin versions
# hadolint ignore=DL3041
RUN microdnf update && \
    microdnf install \
    go \
    gzip \
    tar && \
    microdnf clean all

RUN curl --fail -L "https://repo.stackable.tech/repository/packages/opa/opa_${PRODUCT}.tar.gz" -o opa.tar.gz && \
    tar -zxvf opa.tar.gz && \
    mv opa-${PRODUCT} opa

WORKDIR /opa

RUN go build -o opa -buildmode=exe

# Not tagging base image because it is built as part of the same process
# hadolint ignore=DL3006
FROM stackable/image/vector

ARG PRODUCT
ARG RELEASE

LABEL name="Open Policy Agent" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for OPA." \
      description="This image is deployed by the Stackable Operator for OPA."

RUN microdnf update && \
    microdnf install \
    # Required for filtering logs
    jq && \
    microdnf clean all && \
    rm -rf /var/cache/yum

COPY opa/licenses /licenses

USER stackable
WORKDIR /stackable/opa

COPY --from=opa-builder /opa/opa /stackable/opa/opa
COPY --from=opa-bundle-builder --chown=stackable:stackable /opa-bundle-builder/target/release/stackable-opa-bundle-builder /stackable/opa-bundle-builder
COPY --from=multilog-builder --chown=stackable:stackable /daemontools/admin/daemontools/command/multilog /stackable/multilog

COPY --chown=stackable:stackable opa/stackable/bin /stackable/opa/bin

ENV PATH="${PATH}:/stackable/opa:/stackable/opa/bin"

CMD ["opa", "run", "-s"]
