# syntax=docker/dockerfile:1.15.1@sha256:9857836c9ee4268391bb5b09f9f157f3c91bb15821bb77969642813b0d00518d
# check=error=true

FROM stackable/image/stackable-base
ARG PRODUCT
ARG STACKABLE_USER_UID

WORKDIR /statsd_exporter

RUN --mount=type=cache,id=go-statsd-exporter,uid=${STACKABLE_USER_UID},target=/go_cache <<EOF
microdnf update

# Tar and gzip are used to unpack the statsd_exporter source
# Golang is used to build statsd_exporter
# Git is needed by the cyclonedx-gomod tool to determine the version of statsd_exporter
microdnf install \
  tar \
  gzip \
  git \
  golang

microdnf clean all
rm -rf /var/cache/yum

export GOPATH=/go_cache
# We use version 1.7.0, since a newer version of cyclonedx-gomod is not compatible with the version of Golang (>= 1.23.1)
go install github.com/CycloneDX/cyclonedx-gomod/cmd/cyclonedx-gomod@v1.7.0

curl "https://repo.stackable.tech/repository/packages/statsd_exporter/statsd_exporter-${PRODUCT}.src.tar.gz" | tar -xzC .
(
  cd "statsd_exporter-${PRODUCT}" || exit

  # Unfortunately, we need to create a dummy Git repository to allow cyclonedx-gomod to determine the version of statsd_exporter
  git init
  git config user.email "fake.commiter@stackable.tech"
  git config user.name "Fake commiter"
  git commit --allow-empty --message "Fake commit, so that we can create a tag"
  git tag "v${PRODUCT}"
  go build -o ../statsd_exporter
  $GOPATH/bin/cyclonedx-gomod app -json -output-version 1.5 -output ../statsd_exporter-${PRODUCT}.cdx.json -packages -files
)
rm -rf "statsd_exporter-${PRODUCT}"
EOF
