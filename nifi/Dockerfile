# syntax=docker/dockerfile:1.10.0@sha256:865e5dd094beca432e8c0a1d5e1c465db5f998dca4e439981029b3b81fb39ed5
# check=error=true

FROM stackable/image/java-devel AS nifi-builder

ARG PRODUCT
ARG MAVEN_VERSION="3.9.8"
ARG STACKABLE_USER_UID

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
    curl "https://repo.stackable.tech/repository/packages/maven/apache-maven-${MAVEN_VERSION}-bin.tar.gz" | tar -xzC . && \
    ln -sf /tmp/apache-maven-${MAVEN_VERSION}/bin/mvn /usr/bin/mvn ; \
    fi

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 nifi/stackable/patches /stackable/patches

RUN curl 'https://repo.stackable.tech/repository/m2/tech/stackable/nifi/stackable-bcrypt/1.0-SNAPSHOT/stackable-bcrypt-1.0-20240508.153334-1-jar-with-dependencies.jar' \
    # This used to be located in /bin/stackable-bcrypt.jar. We create a softlink for /bin/stackable-bcrypt.jar in the main container for backwards compatibility.
    -o /stackable/stackable-bcrypt.jar && \
    # Get the source release from nexus
    curl "https://repo.stackable.tech/repository/packages/nifi/nifi-${PRODUCT}-source-release.zip" -o "/stackable/nifi-${PRODUCT}-source-release.zip" && \
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
    mvn --batch-mode --no-transfer-progress clean install -Dmaven.javadoc.skip=true -DskipTests --activate-profiles include-iceberg,include-hadoop-aws,include-hadoop-azure,include-hadoop-gcp && \
    # Copy the binaries to the /stackable folder
    mv /stackable/nifi-${PRODUCT}-src/nifi-assembly/target/nifi-${PRODUCT}-bin/nifi-${PRODUCT} /stackable/nifi-${PRODUCT} && \
    # Copy the SBOM as well
    mv /stackable/nifi-${PRODUCT}-src/nifi-assembly/target/bom.json /stackable/nifi-${PRODUCT}/nifi-${PRODUCT}.cdx.json && \
    # Remove the unzipped sources
    rm -rf /stackable/nifi-${PRODUCT}-src && \
    # Remove generated docs in binary
    rm -rf /stackable/nifi-${PRODUCT}/docs

FROM stackable/image/java-base AS final

ARG PRODUCT
ARG RELEASE
ARG STACKABLE_USER_UID

LABEL name="Apache NiFi" \
    maintainer="info@stackable.tech" \
    vendor="Stackable GmbH" \
    version="${PRODUCT}" \
    release="${RELEASE}" \
    summary="The Stackable image for Apache NiFi." \
    description="This image is deployed by the Stackable Operator for Apache NiFi."

COPY --chown=${STACKABLE_USER_UID}:0 --from=nifi-builder /stackable/nifi-${PRODUCT} /stackable/nifi-${PRODUCT}/
COPY --chown=${STACKABLE_USER_UID}:0 --from=nifi-builder /stackable/stackable-bcrypt.jar /stackable/stackable-bcrypt.jar

COPY --chown=${STACKABLE_USER_UID}:0 nifi/stackable/bin /stackable/bin
COPY --chown=${STACKABLE_USER_UID}:0 nifi/licenses /licenses
COPY --chown=${STACKABLE_USER_UID}:0 nifi/python /stackable/python

RUN <<EOF
ln -s /stackable/nifi-${PRODUCT} /stackable/nifi

microdnf update

# python-pip: Required to install nipyapi
microdnf install \
  python-pip

microdnf clean all
rm -rf /var/cache/yum

# The nipyapi is required for the ReportingTaskJob
pip install --no-cache-dir nipyapi==0.19.1 && \

# For backwards compatibility we create a softlink in /bin where the jar used to be as long as we are root
# This can be removed once older versions / operators using this are no longer supported
ln -s /stackable/stackable-bcrypt.jar /bin/stackable-bcrypt.jar

# All files and folders owned by root group to support running as arbitrary users.
# This is best practice as all container users will belong to the root group (0).
chown -R ${STACKABLE_USER_UID}:0 /stackable
chmod -R g=u /stackable
EOF

# ----------------------------------------
# Attention: We are changing the group of all files in /stackable directly above
# If you do any file based actions (copying / creating etc.) below this comment you
# absolutely need to make sure that the correct permissions are applied!
# chown ${STACKABLE_USER_UID}:0
# ----------------------------------------

USER ${STACKABLE_USER_UID}

ENV HOME=/stackable
ENV NIFI_HOME=/stackable/nifi
ENV PATH="${PATH}":/stackable/nifi/bin

WORKDIR /stackable/nifi
CMD ["bin/nifi.sh", "run"]
