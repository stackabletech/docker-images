# syntax=docker/dockerfile:1.10.0@sha256:865e5dd094beca432e8c0a1d5e1c465db5f998dca4e439981029b3b81fb39ed5
# check=error=true

FROM stackable/image/stackable-base AS opa-bundle-builder

ARG BUNDLE_BUILDER_VERSION

# Update image and install everything needed for Rustup & Rust
RUN microdnf update \
  && microdnf install \
    cmake \
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

# WARNING (@NickLarsenNZ): We should pin the rustup version
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN git clone --depth 1 --branch ${BUNDLE_BUILDER_VERSION} https://github.com/stackabletech/opa-bundle-builder
RUN <<EOF
cd ./opa-bundle-builder
. "$HOME/.cargo/env"
rustup toolchain install
cargo --quiet build --release
# set correct groups
chmod -R g=u /opa-bundle-builder/target/release/
EOF

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

RUN package/install && \
    # set correct groups
    chmod g=u /daemontools/admin/daemontools/command/multilog

FROM stackable/image/stackable-base AS opa-builder

ARG PRODUCT
ARG RELEASE
ARG STACKABLE_USER_UID
ARG TARGETARCH
ARG TARGETOS

ENV GOARCH=$TARGETARCH
ENV GOOS=$TARGETOS

# gzip, tar - used to unpack the OPA source
# git - needed by the cyclonedx-gomod tool to determine the version of OPA
# golang - used to build OPA
RUN microdnf update && \
    microdnf install \
    git \
    golang \
    gzip \
    tar && \
    microdnf clean all

COPY --chown=${STACKABLE_USER_UID}:0 opa/stackable/bin /stackable/opa/bin

RUN <<EOF
# We use version 1.7.0, since a newer version of cyclonedx-gomod is not compatible with the version of Golang (>= 1.23.1)
go install github.com/CycloneDX/cyclonedx-gomod/cmd/cyclonedx-gomod@v1.7.0
curl "https://repo.stackable.tech/repository/packages/opa/opa_${PRODUCT}.tar.gz" -o opa.tar.gz
tar -zxvf opa.tar.gz
mv "opa-${PRODUCT}" opa
EOF

WORKDIR /opa

RUN <<EOF
# Unfortunately, we need to create a dummy Git repository to allow cyclonedx-gomod to determine the version of OPA
git init
git config user.email "fake.commiter@stackable.tech"
git config user.name "Fake commiter"
git commit --allow-empty --message "Fake commit, so that we can create a tag"
git tag "v${PRODUCT}"
go build -o opa -buildmode=exe
# move artifact to /stackable/*/ to copy in final image
~/go/bin/cyclonedx-gomod app -json -output-version 1.5 -output /stackable/opa/"opa_${PRODUCT}.cdx.json" -packages -files
# move artifact to /stackable/* to copy in final image
mv /opa/opa /stackable/opa/
# set correct groups
chmod -R g=u /stackable/opa
EOF

FROM stackable/image/vector

ARG PRODUCT
ARG RELEASE
ARG STACKABLE_USER_UID

LABEL name="Open Policy Agent" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Open Policy Agent (OPA)." \
      description="This image is deployed by the Stackable Operator for OPA."

COPY --chown=${STACKABLE_USER_UID}:0 opa/licenses /licenses

COPY --from=opa-builder --chown=${STACKABLE_USER_UID}:0 /stackable/opa /stackable/opa
COPY --from=opa-bundle-builder --chown=${STACKABLE_USER_UID}:0 /opa-bundle-builder/target/release/stackable-opa-bundle-builder /stackable/opa-bundle-builder
COPY --from=multilog-builder --chown=${STACKABLE_USER_UID}:0 /daemontools/admin/daemontools/command/multilog /stackable/multilog

RUN <<EOF
microdnf update
# jq: Required for filtering logs
microdnf install \
  jq
microdnf clean all
rm -rf /var/cache/yum
EOF

# ----------------------------------------
# Attention:
# If you do any file based actions (copying / creating etc.) below this comment you
# absolutely need to make sure that the correct permissions are applied!
# chown ${STACKABLE_USER_UID}:0
# ----------------------------------------

USER ${STACKABLE_USER_UID}
WORKDIR /stackable/opa

ENV PATH="${PATH}:/stackable/opa:/stackable/opa/bin"

CMD ["opa", "run", "-s"]
