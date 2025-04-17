# syntax=docker/dockerfile:1.10.0@sha256:865e5dd094beca432e8c0a1d5e1c465db5f998dca4e439981029b3b81fb39ed5
# check=error=true

FROM stackable/image/java-devel AS nifi-builder

ARG PRODUCT
ARG MAVEN_VERSION="3.9.8"
ARG STACKABLE_USER_UID

RUN <<EOF
microdnf update
microdnf clean all
rm -rf /var/cache/yum
EOF

# NOTE: From NiFi 2.0.0 upwards Apache Maven 3.9.6+ is required. As of 2024-07-04 the java-devel image
# ships 3.6.3. This will update maven accordingly depending on the version. The error is due to the maven-enforer-plugin.
#
# [ERROR] Rule 2: org.apache.maven.enforcer.rules.version.RequireMavenVersion failed with message:
# [ERROR] Detected Maven Version: 3.6.3 is not in the allowed range [3.9.6,).
#
RUN <<EOF
if [[ "${PRODUCT}" != 1.* ]] ; then
    cd /tmp
    curl "https://repo.stackable.tech/repository/packages/maven/apache-maven-${MAVEN_VERSION}-bin.tar.gz" | tar -xzC .
    ln -sf /tmp/apache-maven-${MAVEN_VERSION}/bin/mvn /usr/bin/mvn
fi
EOF

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 nifi/stackable/patches /stackable/patches

RUN <<EOF
# This used to be located in /bin/stackable-bcrypt.jar. We create a softlink for /bin/stackable-bcrypt.jar in the main container for backwards compatibility.
curl 'https://repo.stackable.tech/repository/m2/tech/stackable/nifi/stackable-bcrypt/1.0-SNAPSHOT/stackable-bcrypt-1.0-20240508.153334-1-jar-with-dependencies.jar' \
    -o /stackable/stackable-bcrypt.jar

# Get the source release from nexus
curl "https://repo.stackable.tech/repository/packages/nifi/nifi-${PRODUCT}-source-release.zip" -o "/stackable/nifi-${PRODUCT}-source-release.zip"
unzip "nifi-${PRODUCT}-source-release.zip"

# Clean up downloaded source after unzipping
rm -rf "nifi-${PRODUCT}-source-release.zip"

# The NiFi "binary" ends up in a folder named "nifi-${PRODUCT}" which should be copied to /stackable
# from /stackable/nifi-${PRODUCT}-src/nifi-assembly/target/nifi-${PRODUCT}-bin/nifi-${PRODUCT} (see later steps)
# Therefore we add the suffix "-src" to be able to copy the binary and remove the unzipped sources afterwards.
mv nifi-${PRODUCT} nifi-${PRODUCT}-src

# Apply patches
chmod +x patches/apply_patches.sh
patches/apply_patches.sh ${PRODUCT}

# Build NiFi
cd /stackable/nifi-${PRODUCT}-src/

# NOTE: Since NiFi 2.0.0 PutIceberg Processor and services were removed, so including the `include-iceberg` profile does nothing.
# Additionally some modules were moved to optional build profiles, so we need to add `include-hadoop` to get `nifi-parquet-nar` for example.
if [[ "${PRODUCT}" != 1.* ]] ; then
    mvn --batch-mode --no-transfer-progress clean install -Dmaven.javadoc.skip=true -DskipTests --activate-profiles include-hadoop,include-hadoop-aws,include-hadoop-azure,include-hadoop-gcp
else
    mvn --batch-mode --no-transfer-progress clean install -Dmaven.javadoc.skip=true -DskipTests --activate-profiles include-iceberg,include-hadoop-aws,include-hadoop-azure,include-hadoop-gcp
fi

# Copy the binaries to the /stackable folder
mv /stackable/nifi-${PRODUCT}-src/nifi-assembly/target/nifi-${PRODUCT}-bin/nifi-${PRODUCT} /stackable/nifi-${PRODUCT}

# Copy the SBOM as well
mv /stackable/nifi-${PRODUCT}-src/nifi-assembly/target/bom.json /stackable/nifi-${PRODUCT}/nifi-${PRODUCT}.cdx.json

# Remove the unzipped sources
rm -rf /stackable/nifi-${PRODUCT}-src

# Remove generated docs in binary
rm -rf /stackable/nifi-${PRODUCT}/docs

# Set correct permissions
chmod -R g=u /stackable
EOF

FROM stackable/image/java-devel AS nifi-iceberg-bundle-builder

ARG NIFI_ICEBERG_BUNDLE
ARG PRODUCT
ARG STACKABLE_USER_UID

USER ${STACKABLE_USER_UID}
WORKDIR /build

RUN <<EOF
mkdir -p /stackable

if [[ "${PRODUCT}" != 1.* ]] ; then
    # curl "https://github.com/stackabletech/nifi-iceberg-bundle/archive/refs/tags/v${NIFI_ICEBERG_BUNDLE}.tar.gz" | tar -xzC .
    curl "https://github.com/stackabletech/nifi-iceberg-bundle/archive/refs/heads/main.tar.gz" | tar -xzC .
    # cd nifi-iceberg-bundle-${NIFI_ICEBERG_BUNDLE}
    cd nifi-iceberg-bundle-main

    mvn \
        --batch-mode \
        --no-transfer-progress\
        clean package \
        -D nifi.version=${PRODUCT} \
        -Dmaven.javadoc.skip=true \
        -Denforcer.skip=true
    # We need "-Denforcer.skip=true", as the Maven version is too old

    cp ./nifi-iceberg-services-api-nar/target/nifi-iceberg-services-api-nar-${NIFI_ICEBERG_BUNDLE}.nar /stackable
    cp ./nifi-iceberg-services-nar/target/nifi-iceberg-services-nar-${NIFI_ICEBERG_BUNDLE}.nar /stackable
    cp ./nifi-iceberg-processors-nar/target/nifi-iceberg-processors-nar-${NIFI_ICEBERG_BUNDLE}.nar /stackable

    cd ..
    # Save disk space, even for intermediate images
    rm -rf nifi-iceberg-bundle-main

# Set correct groups
chmod g=u /stackable/*.nar
fi
EOF

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
COPY --chown=${STACKABLE_USER_UID}:0 --from=nifi-iceberg-bundle-builder /stackable/*.nar /stackable/nifi-${PRODUCT}/lib/

COPY --chown=${STACKABLE_USER_UID}:0 nifi/stackable/bin /stackable/bin
COPY --chown=${STACKABLE_USER_UID}:0 nifi/licenses /licenses
COPY --chown=${STACKABLE_USER_UID}:0 nifi/python /stackable/python

RUN <<EOF
microdnf update

# python-pip: Required to install Python packages
# Note: Python is also required for the Python processors (ExecuteScript and other for NiFi < 2.0 and custom Python processors for NiFi > 2.0)
microdnf install \
  python-pip

microdnf clean all
rm -rf /var/cache/yum

# The nipyapi is required until NiFi 2.0.x for the ReportingTaskJob
# This can be removed once the 1.x.x line is removed
pip install --no-cache-dir \
    nipyapi==0.19.1

# For backwards compatibility we create a softlink in /bin where the jar used to be as long as we are root
# This can be removed once older versions / operators using this are no longer supported
ln -s /stackable/stackable-bcrypt.jar /bin/stackable-bcrypt.jar

ln -s /stackable/nifi-${PRODUCT} /stackable/nifi

# fix missing permissions / ownership
chown --no-dereference ${STACKABLE_USER_UID}:0 /stackable/nifi
chmod --recursive g=u /stackable/python
chmod --recursive g=u /stackable/bin
chmod g=u /stackable/nifi-${PRODUCT}
EOF

# ----------------------------------------
# Checks
# This section is to run final checks to ensure the created final images
# adhere to several minimal requirements like:
# - check file permissions and ownerships
# ----------------------------------------

# Check that permissions and ownership in /stackable are set correctly
# This will fail and stop the build if any mismatches are found.
RUN <<EOF
/bin/check-permissions-ownership.sh /stackable ${STACKABLE_USER_UID} 0
EOF

# ----------------------------------------
# Attention: Do not perform any file based actions (copying/creating etc.) below this comment because the permissions would not be checked.
# ----------------------------------------

USER ${STACKABLE_USER_UID}

ENV HOME=/stackable
ENV NIFI_HOME=/stackable/nifi
ENV PATH="${PATH}:/stackable/nifi/bin"

WORKDIR /stackable/nifi
CMD ["bin/nifi.sh", "run"]
