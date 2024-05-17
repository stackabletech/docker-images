# syntax=docker/dockerfile:1.6.0@sha256:ac85f380a63b13dfcefa89046420e1781752bab202122f8f50032edf31be0021

# Ignoring DL3038 globally because set `assumeyes=True` in dnf.conf in our base image
# Ignoring DL4006 globally because we inherit the SHELL from our base image
# hadolint global ignore=DL3038,DL4006

# Not tagging base image because it is built as part of the same process
# hadolint ignore=DL3006
FROM stackable/image/java-devel AS builder

ARG PRODUCT

RUN microdnf update && \
    microdnf clean all && \
    rm -rf /var/cache/yum

USER stackable
WORKDIR /stackable

COPY --chown=stackable:stackable nifi/stackable/patches /stackable/patches

RUN curl --fail -L 'https://repo.stackable.tech/repository/m2/tech/stackable/nifi/stackable-bcrypt/1.0-SNAPSHOT/stackable-bcrypt-1.0-20240508.153334-1-jar-with-dependencies.jar' \
    # This used to be located in /bin/stackable-bcrypt.jar. We create a softlink for /bin/stackable-bcrypt.jar in the main container for backwards compatibility.
    -o /stackable/stackable-bcrypt.jar && \
    # Get the source release from nexus
    curl --fail -L "https://repo.stackable.tech/repository/packages/nifi/nifi-${PRODUCT}-source-release.zip" -o "/stackable/nifi-${PRODUCT}-source-release.zip" && \
    unzip "nifi-${PRODUCT}-source-release.zip" && \
    # Clean up downloaded source after unzipping
    rm -rf "nifi-${PRODUCT}-source-release.zip" && \
    # The NiFi "binary" ends up in a folder named "nifi-${PRODUCT}" which should be copied to /stackable 
    # from /stackable/nifi-${PRODUCT}-src/nifi-assembly/target/nifi-${PRODUCT}-bin/nifi-${PRODUCT} (see later steps)
    # Therefore we add the suffix "-src" to be able to copy the binary and remove the unzipped sources afterwards.
    mv nifi-${PRODUCT} nifi-${PRODUCT}-src && \
    # Apply patches
    chmod +x patches/apply_patches.sh && \
    patches/apply_patches.sh ${PRODUCT} && \
    # Build NiFi
    cd /stackable/nifi-${PRODUCT}-src/ && \
    mvn clean install -DskipTests && \
    # Copy the binaries to the /stackable folder
    mv /stackable/nifi-${PRODUCT}-src/nifi-assembly/target/nifi-${PRODUCT}-bin/nifi-${PRODUCT} /stackable/nifi-${PRODUCT} && \
    # Remove the unzipped sources
    rm -rf /stackable/nifi-${PRODUCT}-src && \
    # Remove generated docs in binary
    rm -rf /stackable/nifi-${PRODUCT}/docs

# Add Iceberg extensions as they are not included by default and are important enough
# They need to be build from source, as https://mvnrepository.com/artifact/org.apache.nifi/nifi-iceberg-processors-nar does not ship the org.apache.hadoop.fs.s3a.S3AFileSystem (see https://github.com/apache/nifi/pull/6368#issuecomment-1502175258)
# See https://repo.stackable.tech/repository/packages/nifi/iceberg-nars/README.md for details on how to build them
RUN cd /stackable/nifi-${PRODUCT}/lib/ && \
    curl --fail -O "https://repo.stackable.tech/repository/packages/nifi/iceberg-nars/nifi-iceberg-processors-nar-${PRODUCT}-with-aws.nar" && \
    curl --fail -O "https://repo.stackable.tech/repository/packages/nifi/iceberg-nars/nifi-iceberg-services-nar-${PRODUCT}-with-aws.nar" && \
    curl --fail -O "https://repo.stackable.tech/repository/packages/nifi/iceberg-nars/nifi-iceberg-services-api-nar-${PRODUCT}-with-aws.nar"

# ===
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

# Not tagging base image because it is built as part of the same process
# hadolint ignore=DL3006
FROM stackable/image/java-base AS final

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
    # Required to install nipyapi
    python-pip && \
    microdnf clean all && \
    rm -rf /var/cache/yum && \
    # The nipyapi is required for the ReportingTaskJob
    pip install --no-cache-dir nipyapi==0.19.1 && \
    # For backwards compatibility we create a softlink in /bin where the jar used to be as long as we are root
    # This can be removed once older versions / operators using this are no longer supported
    ln -s /stackable/stackable-bcrypt.jar /bin/stackable-bcrypt.jar

USER stackable

COPY --chown=stackable:stackable --from=builder /stackable/nifi-${PRODUCT} /stackable/nifi-${PRODUCT}/
COPY --chown=stackable:stackable --from=builder /stackable/stackable-bcrypt.jar /stackable/stackable-bcrypt.jar

COPY --chown=stackable:stackable nifi/stackable /stackable
COPY --chown=stackable:stackable nifi/licenses /licenses
COPY --chown=stackable:stackable nifi/python /stackable/python

RUN ln -s /stackable/nifi-${PRODUCT} /stackable/nifi

ENV HOME=/stackable
ENV NIFI_HOME=/stackable/nifi
ENV PATH="${PATH}":/stackable/nifi/bin

# ===
# Mitigation for CVE-2021-44228 (Log4Shell)
# This variable is supported as of Log4j version 2.10 and
# disables the vulnerable feature
ENV LOG4J_FORMAT_MSG_NO_LOOKUPS=true
# ===

WORKDIR /stackable/nifi
CMD ["bin/nifi.sh", "run"]
