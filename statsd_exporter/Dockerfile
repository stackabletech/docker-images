# syntax=docker/dockerfile:1.10.0@sha256:865e5dd094beca432e8c0a1d5e1c465db5f998dca4e439981029b3b81fb39ed5
# check=error=true

FROM stackable/image/stackable-base
ARG PRODUCT

WORKDIR /statsd_exporter

RUN --mount=type=cache,id=go-statsd-exporter,uid=1000,target=/go_cache <<EOF
microdnf update

# Tar and gzip are used to unpack the statsd_exporter source
# Golang is used to build statsd_exporter
microdnf install \
  tar \
  gzip \
  golang

microdnf clean all
rm -rf /var/cache/yum

export GOPATH=/go_cache
curl "https://repo.stackable.tech/repository/packages/statsd_exporter/statsd_exporter-${PRODUCT}.src.tar.gz" | tar -xzC .
(
  cd "statsd_exporter-${PRODUCT}" || exit
  go build -o ../statsd_exporter
)
rm -rf "statsd_exporter-${PRODUCT}"
EOF
