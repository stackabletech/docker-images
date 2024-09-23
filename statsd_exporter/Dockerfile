# syntax=docker/dockerfile:1.8.1@sha256:e87caa74dcb7d46cd820352bfea12591f3dba3ddc4285e19c7dcd13359f7cefd

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
