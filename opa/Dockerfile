# syntax=docker/dockerfile:1
FROM stackable/image/stackable-base AS opa-bundle-builder

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

# Update image and install everything needed for Rustup & Rust
RUN microdnf update --disablerepo=* --enablerepo=ubi-8-appstream-rpms --enablerepo=ubi-8-baseos-rpms -y \
  && rm -rf /var/cache/yum \
  && microdnf install --disablerepo=* --enablerepo=ubi-8-appstream-rpms --enablerepo=ubi-8-baseos-rpms curl findutils gcc gcc-c++ git make cmake openssl-devel pkg-config systemd-devel unzip -y \
  && rm -rf /var/cache/yum

WORKDIR /

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN git clone --depth 1 --branch 1.0.0 https://github.com/stackabletech/opa-bundle-builder
RUN cd ./opa-bundle-builder && . $HOME/.cargo/env && cargo build --release

FROM stackable/image/stackable-base

ARG PRODUCT
ARG RELEASE

LABEL name="Open Policy Agent" \
      maintainer="info@stackable.de" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for OPA." \
      description="This image is deployed by the Stackable Operator for OPA."

# https://github.com/hadolint/hadolint/wiki/DL4006
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN microdnf update && \
    microdnf install tar gzip && \
    microdnf install shadow-utils && \
    microdnf clean all

COPY opa/licenses /licenses

USER stackable
WORKDIR /stackable/opa

RUN curl -L https://repo.stackable.tech/repository/packages/opa/opa_${PRODUCT}.tar.gz | tar -xzC . && \
    chmod +x opa
COPY --from=opa-bundle-builder --chown=stackable:stackable /opa-bundle-builder/target/release/stackable-opa-bundle-builder /stackable/opa-bundle-builder

CMD ["./opa", "run", "-s"]
