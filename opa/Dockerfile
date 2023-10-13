# syntax=docker/dockerfile:1
FROM stackable/image/vector AS opa-bundle-builder

ARG BUNDLE_BUILDER_VERSION

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

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

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

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

LABEL name="Open Policy Agent" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for OPA." \
      description="This image is deployed by the Stackable Operator for OPA."

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf install tar gzip && \
    microdnf clean all

COPY opa/licenses /licenses

USER stackable
WORKDIR /stackable/opa

RUN curl --fail -L https://repo.stackable.tech/repository/packages/opa/opa_linux_amd64_static_${PRODUCT} \
     -o opa_linux_amd64_static_${PRODUCT} && \
    chmod +x opa_linux_amd64_static_${PRODUCT} && \
    ln -s opa_linux_amd64_static_${PRODUCT} opa

COPY --from=opa-bundle-builder --chown=stackable:stackable /opa-bundle-builder/target/release/stackable-opa-bundle-builder /stackable/opa-bundle-builder
COPY --from=multilog-builder --chown=stackable:stackable /daemontools/admin/daemontools/command/multilog /stackable/multilog

CMD ["./opa", "run", "-s"]
