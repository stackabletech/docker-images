# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true;skip=InvalidDefaultArgInFrom

ARG GOLANG_VERSION
FROM oci.stackable.tech/sdp/library/golang:${GOLANG_VERSION} AS golang-image

FROM local-image/stackable-devel AS statsd-exporter-builder

ARG PRODUCT_VERSION
ARG CYCLONEDX_GOMOD_VERSION
ARG STACKABLE_USER_UID
ARG TARGETARCH
ARG TARGETOS

ENV CYCLONEDX_GOMOD_VERSION=$CYCLONEDX_GOMOD_VERSION
ENV GOARCH=$TARGETARCH
ENV GOOS=$TARGETOS

WORKDIR /statsd_exporter

RUN <<EOF
microdnf update

# Tar and gzip are used to unpack the statsd_exporter source
# Git is needed by the cyclonedx-gomod tool to determine the version of statsd_exporter
microdnf install \
  tar \
  gzip \
  git

microdnf clean all
rm -rf /var/cache/yum
EOF

# Manually install Go since the dnf package is sometimes not recent enough
COPY --from=golang-image /usr/local/go/ /usr/local/go/
ENV PATH="/usr/local/go/bin:${PATH}"

COPY --chown=${STACKABLE_USER_UID}:0 shared/statsd-exporter/stackable/patches/patchable.toml /statsd_exporter/src/shared/statsd-exporter/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 shared/statsd-exporter/stackable/patches/${PRODUCT_VERSION} /statsd_exporter/src/shared/statsd-exporter/stackable/patches/${PRODUCT_VERSION}

RUN --mount=type=cache,id=go-statsd-exporter,uid=${STACKABLE_USER_UID},target=/go_cache <<EOF
export GOPATH=/go_cache
go install "github.com/CycloneDX/cyclonedx-gomod/cmd/cyclonedx-gomod@v${CYCLONEDX_GOMOD_VERSION}"

(
  cd "$(/stackable/patchable --images-repo-root=src checkout shared/statsd-exporter ${PRODUCT_VERSION})"

  # Create snapshot of the source code including custom patches
  tar -czf /statsd_exporter/statsd-exporter-${PRODUCT_VERSION}-src.tar.gz .

  # Unfortunately, we need to create a dummy Git repository to allow cyclonedx-gomod to determine the version of statsd_exporter
  rm .git
  git init
  git config user.email "fake.commiter@stackable.tech"
  git config user.name "Fake commiter"
  git commit --allow-empty --message "Fake commit, so that we can create a tag"
  git tag "v${PRODUCT_VERSION}"
  go build -o /statsd_exporter/statsd_exporter
  $GOPATH/bin/cyclonedx-gomod app -json -output-version 1.5 -output /statsd_exporter/statsd_exporter-${PRODUCT_VERSION}.cdx.json -packages -files
)
rm -rf "statsd_exporter-${PRODUCT_VERSION}"
# set correct groups
chmod -R g=u /statsd_exporter/
EOF
