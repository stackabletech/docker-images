# syntax=docker/dockerfile:1.10.0@sha256:865e5dd094beca432e8c0a1d5e1c465db5f998dca4e439981029b3b81fb39ed5
# check=error=true

# Normally we would use stackable/image/stackable-base here, *but* we fail to link kcat at the end with error messages
# shown in Snippet 1, which we were not able to solve.

FROM stackable/image/java-base AS builder

ARG PRODUCT
ARG STACKABLE_USER_UID

RUN <<EOF
microdnf update
microdnf install \
    cmake \
    cyrus-sasl-devel \
    gcc-c++ \
    krb5-devel \
    libcurl-devel \
    make \
    openssl-devel \
    tar \
    wget \
    which \
    zlib \
    zlib-devel
microdnf clean all
rm -rf /var/cache/yum
EOF

WORKDIR /stackable

RUN <<EOF
curl -O https://repo.stackable.tech/repository/packages/kcat/kcat-${PRODUCT}.tar.gz
tar xvfz kcat-${PRODUCT}.tar.gz
cd kcat-${PRODUCT}
./bootstrap.sh
# set correct permissions
chmod --recursive g=u /stackable/kcat-${PRODUCT}
EOF

COPY --chown=${STACKABLE_USER_UID}:0 kcat/licenses /licenses

# SNIPPET 1
# 145.2 gcc  -I/stackable/kcat-1.7.0/tmp-bootstrap/usr/include -I/stackable/kcat-1.7.0/tmp-bootstrap/usr/include -g -O2 -Wall -Wsign-compare -Wfloat-equal -Wpointer-arith -Wcast-align -L/stackable/kcat-1.7.0/tmp-bootstrap/usr/lib -Wl,-rpath-link=/stackable/kcat-1.7.0/tmp-bootstrap/usr/lib -L/stackable/kcat-1.7.0/tmp-bootstrap/usr/lib -Wl,-rpath-link=/stackable/kcat-1.7.0/tmp-bootstrap/usr/lib kcat.o format.o tools.o input.o json.o avro.o -o kcat  -lm -ldl -lpthread -lrt -lpthread -lrt  -L/stackable/kcat-1.7.0/tmp-bootstrap/usr/lib  /stackable/kcat-1.7.0/tmp-bootstrap/usr/lib/libavro.a /stackable/kcat-1.7.0/tmp-bootstrap/usr/lib/libjansson.a -lcurl /stackable/kcat-1.7.0/tmp-bootstrap/usr/lib/libserdes.a -Wl,-Bstatic -lavro -Wl,-Bdynamic /stackable/kcat-1.7.0/tmp-bootstrap/usr/lib/libyajl_s.a -L/stackable/kcat-1.7.0/tmp-bootstrap/usr/lib //stackable/kcat-1.7.0/tmp-bootstrap/usr/lib/librdkafka.a -lm -ldl -lpthread -lrt -lz -lcrypto -lssl -lsasl2   -lm -ldl -lpthread -lrt -lpthread -lrt  -L/stackable/kcat-1.7.0/tmp-bootstrap/usr/lib  /stackable/kcat-1.7.0/tmp-bootstrap/usr/lib/libavro.a /stackable/kcat-1.7.0/tmp-bootstrap/usr/lib/libjansson.a -lcurl
# 145.3 /usr/bin/ld: //stackable/kcat-1.7.0/tmp-bootstrap/usr/lib/librdkafka.a(rdkafka_zstd.o): in function `rd_kafka_zstd_decompress':
# 145.3 /stackable/kcat-1.7.0/tmp-bootstrap/librdkafka/src/rdkafka_zstd.c:44: undefined reference to `ZSTD_getFrameContentSize'
# 145.3 /usr/bin/ld: /stackable/kcat-1.7.0/tmp-bootstrap/librdkafka/src/rdkafka_zstd.c:91: undefined reference to `ZSTD_getErrorCode'
# 145.3 /usr/bin/ld: /stackable/kcat-1.7.0/tmp-bootstrap/librdkafka/src/rdkafka_zstd.c:80: undefined reference to `ZSTD_decompress'
# 145.3 /usr/bin/ld: /stackable/kcat-1.7.0/tmp-bootstrap/librdkafka/src/rdkafka_zstd.c:82: undefined reference to `ZSTD_isError'
# 145.3 /usr/bin/ld: /stackable/kcat-1.7.0/tmp-bootstrap/librdkafka/src/rdkafka_zstd.c:100: undefined reference to `ZSTD_getErrorName'
# 145.3 /usr/bin/ld: //stackable/kcat-1.7.0/tmp-bootstrap/usr/lib/librdkafka.a(rdkafka_zstd.o): in function `rd_kafka_zstd_compress':
# 145.3 /stackable/kcat-1.7.0/tmp-bootstrap/librdkafka/src/rdkafka_zstd.c:130: undefined reference to `ZSTD_compressBound'
# 145.3 /usr/bin/ld: /stackable/kcat-1.7.0/tmp-bootstrap/librdkafka/src/rdkafka_zstd.c:141: undefined reference to `ZSTD_createCStream'
# 145.3 /usr/bin/ld: /stackable/kcat-1.7.0/tmp-bootstrap/librdkafka/src/rdkafka_zstd.c:155: undefined reference to `ZSTD_initCStream'
# 145.3 /usr/bin/ld: /stackable/kcat-1.7.0/tmp-bootstrap/librdkafka/src/rdkafka_zstd.c:157: undefined reference to `ZSTD_isError'
# 145.3 /usr/bin/ld: /stackable/kcat-1.7.0/tmp-bootstrap/librdkafka/src/rdkafka_zstd.c:213: undefined reference to `ZSTD_freeCStream'
# 145.3 /usr/bin/ld: /stackable/kcat-1.7.0/tmp-bootstrap/librdkafka/src/rdkafka_zstd.c:168: undefined reference to `ZSTD_compressStream'
# 145.3 /usr/bin/ld: /stackable/kcat-1.7.0/tmp-bootstrap/librdkafka/src/rdkafka_zstd.c:169: undefined reference to `ZSTD_isError'
# 145.3 /usr/bin/ld: /stackable/kcat-1.7.0/tmp-bootstrap/librdkafka/src/rdkafka_zstd.c:170: undefined reference to `ZSTD_getErrorName'
# 145.3 /usr/bin/ld: /stackable/kcat-1.7.0/tmp-bootstrap/librdkafka/src/rdkafka_zstd.c:158: undefined reference to `ZSTD_getErrorName'
# 145.3 /usr/bin/ld: /stackable/kcat-1.7.0/tmp-bootstrap/librdkafka/src/rdkafka_zstd.c:198: undefined reference to `ZSTD_endStream'
# 145.3 /usr/bin/ld: /stackable/kcat-1.7.0/tmp-bootstrap/librdkafka/src/rdkafka_zstd.c:199: undefined reference to `ZSTD_isError'
# 145.3 /usr/bin/ld: /stackable/kcat-1.7.0/tmp-bootstrap/librdkafka/src/rdkafka_zstd.c:213: undefined reference to `ZSTD_freeCStream'
# 145.3 /usr/bin/ld: /stackable/kcat-1.7.0/tmp-bootstrap/librdkafka/src/rdkafka_zstd.c:200: undefined reference to `ZSTD_getErrorName'
# 145.3 collect2: error: ld returned 1 exit status
# 145.3 make: *** [mklove/Makefile.base:225: kcat] Error 1
