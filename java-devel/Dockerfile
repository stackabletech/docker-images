# syntax=docker/dockerfile:1.10.0@sha256:865e5dd094beca432e8c0a1d5e1c465db5f998dca4e439981029b3b81fb39ed5
# check=error=true

#
# Base imaege for builder stages
#

FROM stackable/image/stackable-base

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
