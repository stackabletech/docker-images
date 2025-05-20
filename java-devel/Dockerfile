# syntax=docker/dockerfile:1.15.1@sha256:9857836c9ee4268391bb5b09f9f157f3c91bb15821bb77969642813b0d00518d
# check=error=true

#
# Base image for builder stages in Java based products
#

FROM stackable/image/stackable-devel

ARG PRODUCT
ARG STACKABLE_USER_UID

# See: https://adoptium.net/en-gb/installation/linux/#_centosrhelfedora_instructions
RUN cat <<EOF > /etc/yum.repos.d/adoptium.repo
[Adoptium]
[Adoptium]
name=Adoptium
# The adoptium mirror times-out often, so we have created a pull-through cache
# baseurl=https://packages.adoptium.net/artifactory/rpm/rhel/\$releasever/\$basearch
baseurl=https://build-repo.stackable.tech/repository/Adoptium/\$releasever/\$basearch
enabled=1
gpgcheck=1
gpgkey=https://packages.adoptium.net/artifactory/api/gpg/key/public
EOF

RUN microdnf update && \
    microdnf install -y \
    cmake \
    cyrus-sasl-devel \
    # diff is required by maven during the build of hbase \
    # Cannot run program "diff" (in directory "/stackable/hbase-2.4.12-src/hbase-shaded/hbase-shaded-check-invariants"
    diffutils \
    fuse-devel \
    gcc \
    gcc-c++ \
    # The GNU gettext utilities contain the envsubst program which
    # substitutes the values of environment variables.
    gettext \
    # For the apply_patches.sh script
    git \
    # Needed by the maven ant run plugin for the "set-hostname-property" step in zookeeper
    hostname \
    # Needed for compiling Java projects
    "temurin-${PRODUCT}-jdk" \
    krb5-devel \
    libcurl-devel \
    make \
    maven \
    openssl-devel \
    # Required to unpack Omid tarball
    tar \
    wget \
    which \
    xz \
    zlib-devel \
    # Required for log4shell.sh
    unzip zip && \
    microdnf clean all && \
    rm -rf /var/cache/yum

ENV JAVA_HOME="/usr/lib/jvm/temurin-${PRODUCT}-jdk"

COPY --chown=${STACKABLE_USER_UID}:0 java-devel/stackable/settings.xml /stackable/.m2/settings.xml

# Mitigation for CVE-2021-44228 (Log4Shell)
# This variable is supported as of Log4j version 2.10 and
# disables the vulnerable feature
ENV LOG4J_FORMAT_MSG_NO_LOOKUPS=true
