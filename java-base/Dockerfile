# syntax=docker/dockerfile:1.15.1@sha256:9857836c9ee4268391bb5b09f9f157f3c91bb15821bb77969642813b0d00518d
# check=error=true

#
# Provides the common Java Runtime for SDP products
#
FROM stackable/image/vector

ARG PRODUCT
ARG RELEASE="1"

LABEL name="Stackable image for OpenJDK" \
    maintainer="info@stackable.tech" \
    vendor="Stackable GmbH" \
    version="${PRODUCT}" \
    release="${RELEASE}" \
    summary="The Stackable OpenJDK base image." \
    description="This image is the base image for all Stackable Java product images."

# See: https://adoptium.net/en-gb/installation/linux/#_centosrhelfedora_instructions
RUN cat <<EOF > /etc/yum.repos.d/adoptium.repo
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
    microdnf install \
    # Needed to run Java programs
    "temurin-${PRODUCT}-jre" \
    # Needed, because otherwise e.g. Zookeeper fails with
    # Caused by: java.io.FileNotFoundException: /usr/lib/jvm/java-11-openjdk-11.0.20.0.8-2.el8.x86_64/lib/tzdb.dat (No such file or directory)
    tzdata-java \
    # Most of the Java tools needs at least "klist" and "kinit" for Kerberos integration
    krb5-workstation \
    --nodocs && \
    microdnf clean all

COPY java-base/licenses /licenses

ENV JAVA_HOME="/usr/lib/jvm/temurin-${PRODUCT}-jre"

# This image doesn't include the development packages for Java.
# For images that need the devel package (ex. Spark) use this env variable to
# install the proper javac version.
#
# microdnf install java-${JAVA_VERSION}-openjdk-devel
#
ENV JAVA_VERSION=$PRODUCT

# Mitigation for CVE-2021-44228 (Log4Shell)
# This variable is supported as of Log4j version 2.10 and
# disables the vulnerable feature
ENV LOG4J_FORMAT_MSG_NO_LOOKUPS=true
