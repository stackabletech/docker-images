# syntax=docker/dockerfile:1.6.0@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021

# Ignoring DL3038 globally because set `assumeyes=True` in dnf.conf in our base image
# Ignoring DL4006 globally because we inherit the SHELL from our base image
# hadolint global ignore=DL3038,DL4006

FROM stackable/image/java-base

ARG PRODUCT
ARG RELEASE

LABEL name="Apache NiFi" \
      maintainer="info@stackable.tech" \
      vendor="Stackable GmbH" \
      version="${PRODUCT}" \
      release="${RELEASE}" \
      summary="The Stackable image for Apache NiFi." \
      description="This image is deployed by the Stackable Operator for Apache NiFi."

RUN microdnf update && \
    microdnf install \
    gcc \
    gzip \
    libxml2-devel \
    libxslt-devel \
    tar \
    unzip \
    zip && \
    microdnf install \
    python-devel \
    python-pip \
    python-setuptools && \
    microdnf clean all \
    && rm -rf /var/cache/yum

RUN pip install --no-cache-dir nipyapi==0.19.1

# The bcrypt tool is needed by NiFi to locally encrypt the admin password that is mounted as a secret in cleartext
# The source code is available at https://github.com/stackabletech/stackable-utils/tree/main/stackable-bcrypt
# The binaries are published to Nexus with: mvn clean deploy
# This jar used to be COPYed into the image from a local binary but it's not possible to do so anymore
# because /bin is owned by root and curl cannot write there unless it runs as root.
# This should be fixed by moving the jar to location under /stackable and updating the Nifi operator accordingly.
RUN curl --fail -L 'https://repo.stackable.tech/repository/m2/tech/stackable/nifi/stackable-bcrypt/1.0-SNAPSHOT/stackable-bcrypt-1.0-20240508.153334-1-jar-with-dependencies.jar' -o /bin/stackable-bcrypt.jar \
    && chown stackable:stackable /bin/stackable-bcrypt.jar

USER stackable
WORKDIR /stackable

COPY --chown=stackable:stackable nifi/stackable /stackable
COPY --chown=stackable:stackable nifi/licenses /licenses
COPY --chown=stackable:stackable nifi/python /stackable/python


# zip is different than tar and cannot be just piped, therefore the intermediate save and remove step to unzip
RUN curl --fail -L https://repo.stackable.tech/repository/packages/nifi/nifi-${PRODUCT}-bin.zip -o /stackable/nifi-${PRODUCT}-bin.zip && \
    unzip /stackable/nifi-${PRODUCT}-bin.zip && \
    rm /stackable/nifi-${PRODUCT}-bin.zip && \
    ln -s /stackable/nifi-${PRODUCT} /stackable/nifi

# Add Iceberg extensions as they are not included by default and are important enough
# They need to be build from source, as https://mvnrepository.com/artifact/org.apache.nifi/nifi-iceberg-processors-nar does not ship the org.apache.hadoop.fs.s3a.S3AFileSystem (see https://github.com/apache/nifi/pull/6368#issuecomment-1502175258)
# See https://repo.stackable.tech/repository/packages/nifi/iceberg-nars/README.md for details on how to build them
RUN cd /stackable/nifi/lib/ && \
    curl --fail -O "https://repo.stackable.tech/repository/packages/nifi/iceberg-nars/nifi-iceberg-processors-nar-${PRODUCT}-with-aws.nar" && \
    curl --fail -O "https://repo.stackable.tech/repository/packages/nifi/iceberg-nars/nifi-iceberg-services-nar-${PRODUCT}-with-aws.nar" && \
    curl --fail -O "https://repo.stackable.tech/repository/packages/nifi/iceberg-nars/nifi-iceberg-services-api-nar-${PRODUCT}-with-aws.nar"

# ===
# Mitigation for CVE-2021-44228 (Log4Shell)
# This variable is supported as of Log4j version 2.10 and
# disables the vulnerable feature
ENV LOG4J_FORMAT_MSG_NO_LOOKUPS=true

# For earlier versions this script removes the .class file that contains the
# vulnerable code.
# TODO: This can be restricted to target only versions which do not honor the environment
#   varible that has been set above but this has not currently been implemented
COPY shared/log4shell.sh /bin
RUN /bin/log4shell.sh /stackable/nifi-${PRODUCT}

# Ensure no vulnerable files are left over
# This will currently report vulnerable files being present, as it also alerts on
# SocketNode.class, which we do not remove with our scripts.
# Further investigation will be needed whether this should also be removed.
COPY shared/log4shell_1.6.1-log4shell_Linux_x86_64 /bin/log4shell_scanner_x86_64
COPY shared/log4shell_1.6.1-log4shell_Linux_aarch64 /bin/log4shell_scanner_aarch64
COPY shared/log4shell_scanner /bin/log4shell_scanner
RUN /bin/log4shell_scanner s /stackable/nifi-${PRODUCT}
# ===

WORKDIR /stackable/nifi
CMD ["bin/nifi.sh", "run"]
