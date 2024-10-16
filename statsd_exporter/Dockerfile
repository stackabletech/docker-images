# syntax=docker/dockerfile:1.10.0@sha256:865e5dd094beca432e8c0a1d5e1c465db5f998dca4e439981029b3b81fb39ed5
# check=error=true

FROM stackable/image/stackable-base
ARG PRODUCT

WORKDIR /statsd_exporter

RUN --mount=type=cache,id=go-statsd-exporter,uid=1000,target=/go_cache <<EOF
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
  git add go.mod
  git config --global user.email "dummy@stackable.tech"
  git config --global user.name "dummy"
  git commit -m "dummy"
  git tag "${PRODUCT}"
  go build -o ../statsd_exporter
  /go_cache/bin/cyclonedx-gomod app -json -output-version 1.5 -output ../statsd_exporter-${PRODUCT}.cdx.json -packages -files
)
rm -rf "statsd_exporter-${PRODUCT}"
EOF
