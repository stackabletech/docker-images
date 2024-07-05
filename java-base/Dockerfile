# syntax=docker/dockerfile:1.8.1@sha256:e87caa74dcb7d46cd820352bfea12591f3dba3ddc4285e19c7dcd13359f7cefd
#
# Provides the common Java Runtime for SDP products
#
# Ignoring DL3038 globally because set `assumeyes=True` in dnf.conf in our base image
# Ignoring DL4006 globally because we inherit the SHELL from our base image
# hadolint global ignore=DL3038,DL4006

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

RUN microdnf update && \
    microdnf install \
    # Obviously needed to run Java programs
    java-${PRODUCT}-openjdk-headless \
    # Needed, because otherwise e.g. Zookeeper fails with
    # Caused by: java.io.FileNotFoundException: /usr/lib/jvm/java-11-openjdk-11.0.20.0.8-2.el8.x86_64/lib/tzdb.dat (No such file or directory)
    tzdata-java \
    # Most of the Java tools needs at least "klist" and "kinit" for Kerberos integration
    krb5-workstation \
    --nodocs && \
    microdnf clean all

COPY java-base/licenses /licenses

ENV JAVA_HOME=/usr/lib/jvm/jre-${PRODUCT}

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
