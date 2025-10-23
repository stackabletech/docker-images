# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true;skip=InvalidDefaultArgInFrom

ARG GIT_SYNC_VERSION

FROM oci.stackable.tech/sdp/git-sync/git-sync:${GIT_SYNC_VERSION} AS git-sync-image

FROM local-image/shared/logback AS patched-logback

FROM local-image/java-devel AS nifi-builder

ARG PRODUCT_VERSION
ARG RELEASE_VERSION
ARG STACKABLE_USER_UID

RUN <<EOF
microdnf update
microdnf clean all
rm -rf /var/cache/yum
EOF

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 nifi/stackable/patches/patchable.toml /stackable/src/nifi/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 nifi/stackable/patches/${PRODUCT_VERSION} /stackable/src/nifi/stackable/patches/${PRODUCT_VERSION}
COPY --chown=${STACKABLE_USER_UID}:0 --from=git-sync-image /git-sync /stackable/git-sync
COPY --chown=${STACKABLE_USER_UID}:0 --from=patched-logback /stackable/.m2/repository /stackable/patched-logback-libs

RUN <<EOF
mkdir -p /stackable/.m2/repository
cp -r /stackable/patched-logback-libs/* /stackable/.m2/repository

# This used to be located in /bin/stackable-bcrypt.jar. We create a softlink for /bin/stackable-bcrypt.jar in the main container for backwards compatibility.
curl 'https://repo.stackable.tech/repository/m2/tech/stackable/nifi/stackable-bcrypt/1.0-SNAPSHOT/stackable-bcrypt-1.0-20240508.153334-1-jar-with-dependencies.jar' \
    -o /stackable/stackable-bcrypt.jar

cd "$(/stackable/patchable --images-repo-root=src checkout nifi ${PRODUCT_VERSION})"

ORIGINAL_VERSION=$(mvn help:evaluate -Dexpression=project.version -q -DforceStdout)
NEW_VERSION="${PRODUCT_VERSION}-stackable${RELEASE_VERSION}"

mvn versions:set -DnewVersion=$NEW_VERSION

# Create snapshot of the source code including custom patches
tar -czf /stackable/nifi-${NEW_VERSION}-src.tar.gz .

# NOTE: Since NiFi 2.0.0 PutIceberg Processor and services were removed, so including the `include-iceberg` profile does nothing.
# Additionally some modules were moved to optional build profiles, so we need to add `include-hadoop` to get `nifi-parquet-nar` for example.
if [[ "${PRODUCT_VERSION}" != 1.* ]] ; then
    mvn \
    clean install \
    -Dmaven.javadoc.skip=true \
    -DskipTests \
    --activate-profiles include-hadoop,include-hadoop-aws,include-hadoop-azure,include-hadoop-gcp
else
    mvn \
    clean install \
    -Dmaven.javadoc.skip=true \
    -DskipTests \
    --activate-profiles include-iceberg,include-hadoop-aws,include-hadoop-azure,include-hadoop-gcp
fi

# Copy the binaries to the /stackable folder
mv nifi-assembly/target/nifi-${NEW_VERSION}-bin/nifi-${NEW_VERSION} /stackable/nifi-${NEW_VERSION}

# Copy the SBOM as well
sed -i "s/${NEW_VERSION}/${ORIGINAL_VERSION}/g" nifi-assembly/target/bom.json
mv nifi-assembly/target/bom.json /stackable/nifi-${NEW_VERSION}/nifi-${NEW_VERSION}.cdx.json

# Get a list of NARs
export NARS=$(ls /stackable/nifi-${NEW_VERSION}/lib/*.nar | awk -F '/' '{ print $5 }' | sed "s/\-${NEW_VERSION}.nar\$//g")
# Get a list of SBOMs
find . -name bom.json > bomlist.txt

for nar in $NARS; do
    # Try to find the NAR in the list of SBOMs
    match=$(grep "\/$nar\/target\/bom.json" bomlist.txt || true)
    if [[ -n "$match" ]]; then
        # Copy the SBOM of the NAR
        cp "$match" "/stackable/nifi-${NEW_VERSION}/$nar.cdx.json"
    fi
done

# Remove sources
(cd .. && rm -r ${PRODUCT_VERSION})

# Remove generated docs in binary
rm -rf /stackable/nifi-${NEW_VERSION}/docs

# Set correct permissions
chmod -R g=u /stackable
EOF

FROM local-image/java-devel AS nifi-iceberg-bundle-builder

ARG NIFI_ICEBERG_BUNDLE_VERSION
ARG PRODUCT_VERSION
ARG STACKABLE_USER_UID

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 nifi/nifi-iceberg-bundle/stackable/patches/patchable.toml /stackable/src/nifi/nifi-iceberg-bundle/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 nifi/nifi-iceberg-bundle/stackable/patches/${NIFI_ICEBERG_BUNDLE_VERSION} /stackable/src/nifi/nifi-iceberg-bundle/stackable/patches/${NIFI_ICEBERG_BUNDLE_VERSION}

RUN <<EOF
mkdir -p /stackable

# NiFI 1.x natively supports Iceberg, no need to build an iceberg-bundle for it
if [[ "${PRODUCT_VERSION}" != 1.* ]] ; then
    cd "$(/stackable/patchable --images-repo-root=src checkout nifi/nifi-iceberg-bundle ${NIFI_ICEBERG_BUNDLE_VERSION})"

    # Create snapshot of the source code including custom patches
    tar -czf /stackable/nifi-iceberg-bundle-${NIFI_ICEBERG_BUNDLE_VERSION}-src.tar.gz .

    sed -i -e "s/{{ NIFI_VERSION }}/${PRODUCT_VERSION}/g" pom.xml

    mvn \
        clean package \
        -D nifi.version=${PRODUCT_VERSION} \
        -Dmaven.javadoc.skip=true \
        -Denforcer.skip=true
    # We need "-Denforcer.skip=true", as the Maven version is too old

    cp ./nifi-iceberg-services-api-nar/target/nifi-iceberg-services-api-nar-${NIFI_ICEBERG_BUNDLE_VERSION}.nar /stackable
    cp ./nifi-iceberg-services-nar/target/nifi-iceberg-services-nar-${NIFI_ICEBERG_BUNDLE_VERSION}.nar /stackable
    cp ./nifi-iceberg-processors-nar/target/nifi-iceberg-processors-nar-${NIFI_ICEBERG_BUNDLE_VERSION}.nar /stackable
    cp ./target/bom.json /stackable/nifi-iceberg-bundle-${NIFI_ICEBERG_BUNDLE_VERSION}.cdx.json

    # Save disk space, even for intermediate images
    (cd .. && rm -r ${NIFI_ICEBERG_BUNDLE_VERSION})

# Set correct groups
chmod g=u /stackable/*.nar
chmod g=u /stackable/*.cdx.json
fi
EOF

FROM local-image/java-devel AS opa-authorizer-builder

ARG NIFI_OPA_AUTHORIZER_PLUGIN_VERSION
ARG STACKABLE_USER_UID
ARG PRODUCT_VERSION

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 nifi/opa-plugin/stackable/patches/patchable.toml /stackable/src/nifi/opa-plugin/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 nifi/opa-plugin/stackable/patches/${NIFI_OPA_AUTHORIZER_PLUGIN_VERSION} /stackable/src/nifi/opa-plugin/stackable/patches/${NIFI_OPA_AUTHORIZER_PLUGIN_VERSION}

RUN <<EOF
mkdir -p /stackable

cd "$(/stackable/patchable --images-repo-root=src checkout nifi/opa-plugin ${NIFI_OPA_AUTHORIZER_PLUGIN_VERSION})/authorizer"

# Create snapshot of the source code including custom patches
tar -czf /stackable/nifi-opa-plugin-${NIFI_OPA_AUTHORIZER_PLUGIN_VERSION}-src.tar.gz .

mvn \
    clean package \
    -DskipTests \
    -Pnifi-${PRODUCT_VERSION}

cp ./target/opa-authorizer.nar /stackable/opa-authorizer.nar
cp ../LICENSE /stackable/LICENSE

# Set correct permissions
chmod g=u /stackable/opa-authorizer.nar
EOF

FROM local-image/java-base AS final

ARG PRODUCT_VERSION
ARG RELEASE_VERSION
ARG STACKABLE_USER_UID
ARG NIFI_OPA_AUTHORIZER_PLUGIN_VERSION

LABEL name="Apache NiFi" \
    maintainer="info@stackable.tech" \
    vendor="Stackable GmbH" \
    version="${PRODUCT_VERSION}" \
    release="${RELEASE_VERSION}" \
    summary="The Stackable image for Apache NiFi." \
    description="This image is deployed by the Stackable Operator for Apache NiFi."

COPY --chown=${STACKABLE_USER_UID}:0 --from=nifi-builder /stackable/nifi-${PRODUCT_VERSION}-stackable${RELEASE_VERSION} /stackable/nifi-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}/
COPY --chown=${STACKABLE_USER_UID}:0 --from=nifi-builder /stackable/nifi-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-src.tar.gz /stackable
COPY --chown=${STACKABLE_USER_UID}:0 --from=nifi-builder /stackable/stackable-bcrypt.jar /stackable/stackable-bcrypt.jar
COPY --chown=${STACKABLE_USER_UID}:0 --from=nifi-iceberg-bundle-builder /stackable/*.nar /stackable/nifi-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}/lib/
COPY --chown=${STACKABLE_USER_UID}:0 --from=nifi-iceberg-bundle-builder /stackable/*.cdx.json /stackable/nifi-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}/lib/
COPY --chown=${STACKABLE_USER_UID}:0 --from=nifi-iceberg-bundle-builder /stackable/*-src.tar.gz /stackable
COPY --chown=${STACKABLE_USER_UID}:0 --from=nifi-builder /stackable/git-sync /stackable/git-sync
COPY --chown=${STACKABLE_USER_UID}:0 --from=patched-logback /stackable/*-src.tar.gz /stackable

COPY --chown=${STACKABLE_USER_UID}:0 --from=opa-authorizer-builder /stackable/opa-authorizer.nar /stackable/nifi-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}/extensions/opa-authorizer.nar
COPY --chown=${STACKABLE_USER_UID}:0 --from=opa-authorizer-builder /stackable/nifi-opa-plugin-${NIFI_OPA_AUTHORIZER_PLUGIN_VERSION}-src.tar.gz  /stackable
COPY --chown=${STACKABLE_USER_UID}:0 --from=opa-authorizer-builder /stackable/LICENSE /licenses/NIFI_OPA_PLUGIN_LICENSE
COPY --chown=${STACKABLE_USER_UID}:0 nifi/stackable/bin /stackable/bin
COPY --chown=${STACKABLE_USER_UID}:0 nifi/licenses /licenses
COPY --chown=${STACKABLE_USER_UID}:0 nifi/python /stackable/python

# The nipyapi is required until NiFi 2.0.x for the ReportingTaskJob
# This can be removed once the 1.x.x line is removed
# Note: Keep this in sync with the version installed in testing-tools! (See the requirements.txt file)
# Find the latest version here: https://pypi.org/project/nipyapi/
# renovate: datasource=pypi packageName=nipyapi
ENV NIPYAPI_VERSION=0.22.0

RUN <<EOF
microdnf update

# git: Required for git-sync
# python-pip: Required to install Python packages
# Note: Python is also required for the Python processors (ExecuteScript and other for NiFi < 2.0 and custom Python processors for NiFi > 2.0)
microdnf install \
  git \
  python-pip

microdnf clean all
rm -rf /var/cache/yum

pip install --no-cache-dir \
    nipyapi==${NIPYAPI_VERSION}

# For backwards compatibility we create a softlink in /bin where the jar used to be as long as we are root
# This can be removed once older versions / operators using this are no longer supported
ln -s /stackable/stackable-bcrypt.jar /bin/stackable-bcrypt.jar

ln -s /stackable/nifi-${PRODUCT_VERSION}-stackable${RELEASE_VERSION} /stackable/nifi

# fix missing permissions / ownership
chown --no-dereference ${STACKABLE_USER_UID}:0 /stackable/nifi
chmod --recursive g=u /stackable/python
chmod --recursive g=u /stackable/bin
chmod g=u /stackable/nifi-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}
chmod g=u /stackable/*-src.tar.gz
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
