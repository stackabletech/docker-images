# syntax=docker/dockerfile:1.6.0@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021

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
    # Includes tools such as jps or jmap (which is e.g. needed by the Spark UI, see https://github.com/stackabletech/spark-k8s-operator/discussions/340)
    # We consider them useful enough to justify putting them in all Java images
    java-${PRODUCT}-openjdk-devel \
    # Needed, because otherwise e.g. Zookeeper fails with
    # Caused by: java.io.FileNotFoundException: /usr/lib/jvm/java-11-openjdk-11.0.20.0.8-2.el8.x86_64/lib/tzdb.dat (No such file or directory)
    tzdata-java \
    # Most of the Java tools needs at least "klist" and "kinit" for Kerberos integration
    krb5-workstation \
    --nodocs && \
    microdnf clean all

COPY java-base/licenses /licenses

ENV JAVA_HOME=/usr/lib/jvm/jre-${PRODUCT}
