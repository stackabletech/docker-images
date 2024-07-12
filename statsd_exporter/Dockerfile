FROM stackable/image/stackable-base
ARG PRODUCT
RUN microdnf update && \
    microdnf install \
        tar \
        gzip \
        golang && \
    microdnf clean all && \
    rm -rf /var/cache/yum && \
    mkdir -p /statsd_exporter
WORKDIR /statsd_exporter

RUN curl --fail -L "https://repo.stackable.tech/repository/packages/statsd_exporter/statsd_exporter-${PRODUCT}.src.tar.gz" | \
    tar -xzC . && cd "statsd_exporter-${PRODUCT}" && go build -o ../statsd_exporter && cd .. && \
    rm -rf "statsd_exporter-${PRODUCT}"