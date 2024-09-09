# syntax=docker/dockerfile:1.8.1@sha256:e87caa74dcb7d46cd820352bfea12591f3dba3ddc4285e19c7dcd13359f7cefd

FROM stackable/image/java-devel AS nifi-builder

ARG PRODUCT
ARG MAVEN_VERSION="3.9.8"

RUN microdnf update && \
    microdnf clean all && \
    rm -rf /var/cache/yum

# NOTE: From NiFi 2.0.0 upwards Apache Maven 3.9.6+ is required. As of 2024-07-04 the java-devel image
# ships 3.6.3. This will update maven accordingly depending on the version. The error is due to the maven-enforer-plugin.
#
# [ERROR] Rule 2: org.apache.maven.enforcer.rules.version.RequireMavenVersion failed with message:
# [ERROR] Detected Maven Version: 3.6.3 is not in the allowed range [3.9.6,).
#
WORKDIR /tmp
RUN if [[ "${PRODUCT}" == 2.* ]] ; then \
        curl --fail -L "https://repo.stackable.tech/repository/packages/maven/apache-maven-${MAVEN_VERSION}-bin.tar.gz" | tar -xzC . && \
        ln -sf /tmp/apache-maven-${MAVEN_VERSION}/bin/mvn /usr/bin/mvn ; \
    fi

USER stackable
WORKDIR /stackable

COPY --chown=stackable:stackable nifi/stackable/patches /stackable/patches

# NOTE: NiFi 1.21.0 source build does not work with the current arm64 git runners due to java heap issues:
#
# [ERROR] Failed to execute goal org.apache.maven.plugins:maven-assembly-plugin:3.5.0:single (make shared resource) on project nifi-registry-assembly:
# Failed to create assembly: Error creating assembly archive bin: Problem creating zip: Execution exception: Java heap space
#
# Since this will be deprecated in the release 24.7 and then removed we copy the NiFi 1.21.0 binaries instead
# of building from source. The if condition can be removed once 1.21.0 is no longer supported and only the
# else branch is required to build from source.
#
RUN if [[ "${PRODUCT}" == "1.21.0" ]] ; then \
        curl --fail -L 'https://repo.stackable.tech/repository/m2/tech/stackable/nifi/stackable-bcrypt/1.0-SNAPSHOT/stackable-bcrypt-1.0-20240508.153334-1-jar-with-dependencies.jar' \
        # This used to be located in /bin/stackable-bcrypt.jar. We create a softlink for /bin/stackable-bcrypt.jar in the main container for backwards compatibility.
        -o /stackable/stackable-bcrypt.jar && \
        # zip is different than tar and cannot be just piped, therefore the intermediate save and remove step to unzip
        curl --fail -L https://repo.stackable.tech/repository/packages/nifi/nifi-${PRODUCT}-bin.zip -o /stackable/nifi-${PRODUCT}-bin.zip && \
        unzip /stackable/nifi-${PRODUCT}-bin.zip && \
        rm /stackable/nifi-${PRODUCT}-bin.zip && \
        # Remove generated docs in binary
        rm -rf /stackable/nifi-${PRODUCT}/docs && \
        # Add Iceberg extensions as they are not included by default and are important enough
        # They need to be build from source, as https://mvnrepository.com/artifact/org.apache.nifi/nifi-iceberg-processors-nar does not ship the org.apache.hadoop.fs.s3a.S3AFileSystem (see https://github.com/apache/nifi/pull/6368#issuecomment-1502175258)
        # See https://repo.stackable.tech/repository/packages/nifi/iceberg-nars/README.md for details on how to build them
        cd /stackable/nifi-${PRODUCT}/lib/ && \
        curl --fail -O "https://repo.stackable.tech/repository/packages/nifi/iceberg-nars/nifi-iceberg-processors-nar-${PRODUCT}-with-aws.nar" && \
        curl --fail -O "https://repo.stackable.tech/repository/packages/nifi/iceberg-nars/nifi-iceberg-services-nar-${PRODUCT}-with-aws.nar" && \
        curl --fail -O "https://repo.stackable.tech/repository/packages/nifi/iceberg-nars/nifi-iceberg-services-api-nar-${PRODUCT}-with-aws.nar" ; \
    else \
        curl --fail -L 'https://repo.stackable.tech/repository/m2/tech/stackable/nifi/stackable-bcrypt/1.0-SNAPSHOT/stackable-bcrypt-1.0-20240508.153334-1-jar-with-dependencies.jar' \
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
        mvn clean install -Dmaven.javadoc.skip=true -DskipTests --activate-profiles include-iceberg,include-hadoop-aws && \
        # Copy the binaries to the /stackable folder
        mv /stackable/nifi-${PRODUCT}-src/nifi-assembly/target/nifi-${PRODUCT}-bin/nifi-${PRODUCT} /stackable/nifi-${PRODUCT} && \
        # Copy the SBOM as well
        mv /stackable/nifi-${PRODUCT}-src/nifi-assembly/target/bom.json /stackable/nifi-${PRODUCT}/nifi-${PRODUCT}.cdx.json && \
        # Remove the unzipped sources
        rm -rf /stackable/nifi-${PRODUCT}-src && \
        # Remove generated docs in binary
        rm -rf /stackable/nifi-${PRODUCT}/docs ; \
    fi

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

COPY --chown=stackable:stackable --from=nifi-builder /stackable/nifi-${PRODUCT} /stackable/nifi-${PRODUCT}/
COPY --chown=stackable:stackable --from=nifi-builder /stackable/stackable-bcrypt.jar /stackable/stackable-bcrypt.jar

COPY --chown=stackable:stackable nifi/stackable/bin /stackable/bin
COPY --chown=stackable:stackable nifi/licenses /licenses
COPY --chown=stackable:stackable nifi/python /stackable/python

RUN ln -s /stackable/nifi-${PRODUCT} /stackable/nifi

ENV HOME=/stackable
ENV NIFI_HOME=/stackable/nifi
ENV PATH="${PATH}":/stackable/nifi/bin

WORKDIR /stackable/nifi
CMD ["bin/nifi.sh", "run"]
