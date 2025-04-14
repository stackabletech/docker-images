# syntax=docker/dockerfile:1.10.0@sha256:865e5dd094beca432e8c0a1d5e1c465db5f998dca4e439981029b3b81fb39ed5
# check=error=true

FROM stackable/image/hadoop AS hadoop-builder

FROM stackable/image/java-devel AS hive-builder

# Apache Hive up to 4.0.x(!) officially requires Java 8 (there is no distinction between building and running).
# As of 2024-04-15 we for sure need Java 8 for building, but we used a Java 11 runtime for months now without any problems.
# As we got weird TLS errors (https://stackable-workspace.slack.com/archives/C031A5BEFS7/p1713185172557459) with a
# Java 8 runtime we bumped the Runtime to Java 11 again.

ARG PRODUCT
ARG HADOOP
ARG JMX_EXPORTER
ARG AWS_JAVA_SDK_BUNDLE
ARG AZURE_STORAGE
ARG AZURE_KEYVAULT_CORE
ARG STACKABLE_USER_UID

# Setting this to anything other than "true" will keep the cache folders around (e.g. for Maven, NPM etc.)
# This can be used to speed up builds when disk space is of no concern.
ARG DELETE_CACHES="true"

# Copy patches into the builder
COPY --chown=${STACKABLE_USER_UID}:0 hive/stackable/patches/${PRODUCT} /stackable/src/hive/stackable/patches/${PRODUCT}
# It is useful to see which version of Hadoop is used at a glance
# Therefore the use of the full name here
# TODO: Do we really need all of Hadoop in here?
COPY --chown=${STACKABLE_USER_UID}:0 --from=hadoop-builder /stackable/hadoop /stackable/hadoop-${HADOOP}

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

# Cache mounts are owned by root by default
# We need to explicitly give the uid to use
RUN --mount=type=cache,id=maven-hive-${PRODUCT},uid=${STACKABLE_USER_UID},target=/stackable/.m2/repository <<EOF
BUILD_SRC_DIR="$(/stackable/patchable --images-repo-root=src checkout hive ${PRODUCT})"
cd "$BUILD_SRC_DIR"

if [[ "${PRODUCT}" == "3.1.3" ]] ; then
  mvn --batch-mode --no-transfer-progress clean package -DskipTests --projects standalone-metastore
  mv standalone-metastore/target/apache-hive-metastore-${PRODUCT}-bin/apache-hive-metastore-${PRODUCT}-bin /stackable
  mv standalone-metastore/target/bom.json /stackable/apache-hive-metastore-${PRODUCT}-bin/apache-hive-metastore-${PRODUCT}.cdx.json
else
  (
    # https://issues.apache.org/jira/browse/HIVE-20451 switched the metastore server packaging starting with 4.0.0
    cd standalone-metastore
    mvn --batch-mode --no-transfer-progress clean package -DskipTests --projects metastore-server

    # We only seem to get a .tar.gz archive, so let's extract that to the correct location
    tar --extract --directory=/stackable -f metastore-server/target/apache-hive-standalone-metastore-server-${PRODUCT}-bin.tar.gz
    mv metastore-server/target/bom.json /stackable/apache-hive-metastore-${PRODUCT}-bin/apache-hive-metastore-${PRODUCT}.cdx.json

    # TODO: Remove once the fix https://github.com/apache/hive/pull/5419 is merged and released
    # The schemaTool.sh is still pointing to the class location from Hive < 4.0.0, it seems like it was forgotten to update it
    sed -i -e 's/CLASS=org.apache.hadoop.hive.metastore.tools.MetastoreSchemaTool/CLASS=org.apache.hadoop.hive.metastore.tools.schematool.MetastoreSchemaTool/' /stackable/apache-hive-metastore-${PRODUCT}-bin/bin/ext/schemaTool.sh
  )
fi

# Remove sourcecode
cd /stackable
rm -rf "$BUILD_SRC_DIR"

mkdir /stackable/jmx
curl "https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" -o "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar"
ln -s "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" /stackable/jmx/jmx_prometheus_javaagent.jar

# The next two sections for S3 and Azure use hardcoded version numbers on purpose instead of wildcards
# This way the build will fail should one of the files not be available anymore in a later Hadoop version!

# Add S3 Support for Hive (support for s3a://)
cp /stackable/hadoop-${HADOOP}/share/hadoop/tools/lib/hadoop-aws-${HADOOP}.jar /stackable/apache-hive-metastore-${PRODUCT}-bin/lib/
cp /stackable/hadoop-${HADOOP}/share/hadoop/tools/lib/aws-java-sdk-bundle-${AWS_JAVA_SDK_BUNDLE}.jar /stackable/apache-hive-metastore-${PRODUCT}-bin/lib/

# Add Azure ABFS support (support for abfs://)
cp /stackable/hadoop-${HADOOP}/share/hadoop/tools/lib/hadoop-azure-${HADOOP}.jar /stackable/apache-hive-metastore-${PRODUCT}-bin/lib/
cp /stackable/hadoop-${HADOOP}/share/hadoop/tools/lib/azure-storage-${AZURE_STORAGE}.jar /stackable/apache-hive-metastore-${PRODUCT}-bin/lib/
cp /stackable/hadoop-${HADOOP}/share/hadoop/tools/lib/azure-keyvault-core-${AZURE_KEYVAULT_CORE}.jar /stackable/apache-hive-metastore-${PRODUCT}-bin/lib/

# We're removing these to make the intermediate layer smaller
# This can be necessary even though it's only a builder image because the GitHub Action Runners only have very limited space available
# and we are sometimes running into errors because we're out of space.
# Therefore, we try to clean up all layers as much as possible.
if [ "${DELETE_CACHES}" = "true" ] ; then
  rm -rf /stackable/.m2/repository/*
  rm -rf /stackable/.npm/*
  rm -rf /stackable/.cache/*
fi

# change groups
chmod --recursive g=u /stackable
EOF


FROM stackable/image/java-base AS final

ARG PRODUCT
ARG HADOOP
ARG RELEASE
ARG STACKABLE_USER_UID


ARG NAME="Apache Hive metastore"
ARG DESCRIPTION="This image is deployed by the Stackable Operator for Apache Hive."

LABEL name="Apache Hive metastore"
LABEL version="${PRODUCT}"
LABEL release="${RELEASE}"
LABEL summary="The Stackable image for Apache Hive metastore."
LABEL description="${DESCRIPTION}"

# https://github.com/opencontainers/image-spec/blob/036563a4a268d7c08b51a08f05a02a0fe74c7268/annotations.md#annotations
LABEL org.opencontainers.image.documentation="https://docs.stackable.tech/home/stable/hive/"
LABEL org.opencontainers.image.version="${PRODUCT}"
LABEL org.opencontainers.image.revision="${RELEASE}"
LABEL org.opencontainers.image.title="${NAME}"
LABEL org.opencontainers.image.description="${DESCRIPTION}"

# https://docs.openshift.com/container-platform/4.16/openshift_images/create-images.html#defining-image-metadata
# https://github.com/projectatomic/ContainerApplicationGenericLabels/blob/master/vendor/redhat/labels.md
LABEL io.openshift.tags="ubi9,stackable,hive,sdp"
LABEL io.k8s.description="${DESCRIPTION}"
LABEL io.k8s.display-name="${NAME}"

WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 --from=hive-builder /stackable/apache-hive-metastore-${PRODUCT}-bin /stackable/apache-hive-metastore-${PRODUCT}-bin
COPY --chown=${STACKABLE_USER_UID}:0 --from=hive-builder /stackable/hadoop-${HADOOP} /stackable/hadoop-${HADOOP}
COPY --chown=${STACKABLE_USER_UID}:0 --from=hive-builder /stackable/jmx /stackable/jmx
COPY --chown=${STACKABLE_USER_UID}:0 hive/stackable/jmx /stackable/jmx
COPY --chown=${STACKABLE_USER_UID}:0 hive/stackable/bin/start-metastore /stackable/apache-hive-metastore-${PRODUCT}-bin/bin

COPY hive/licenses /licenses

RUN <<EOF
microdnf update
microdnf clean all
rpm -qa --qf "%{NAME}-%{VERSION}-%{RELEASE}\n" | sort > /stackable/package_manifest.txt
chown ${STACKABLE_USER_UID}:0 /stackable/package_manifest.txt
chmod g=u /stackable/package_manifest.txt
rm -rf /var/cache/yum

chmod g=u /stackable/apache-hive-metastore-${PRODUCT}-bin/bin/start-metastore

ln -s /stackable/apache-hive-metastore-${PRODUCT}-bin /stackable/hive-metastore
chown -h ${STACKABLE_USER_UID}:0 /stackable/hive-metastore
chmod g=u /stackable/hive-metastore
ln -s /stackable/hadoop-${HADOOP} /stackable/hadoop
chown -h ${STACKABLE_USER_UID}:0 /stackable/hadoop
chmod g=u /stackable/hadoop

# fix missing permissions
chmod --recursive g=u /stackable/jmx
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

ENV HADOOP_HOME=/stackable/hadoop
ENV HIVE_HOME=/stackable/hive-metastore
ENV PATH="${PATH}":/stackable/hadoop/bin:/stackable/hive-metastore/bin

# The following 2 env-vars are required for common hadoop scripts even if the respective libraries are never used.
# We set them here to a sensible default.
ENV HADOOP_YARN_HOME=/stackable/hadoop
ENV HADOOP_MAPRED_HOME=/stackable/hadoop

WORKDIR /stackable/hive-metastore
# Start command is set by operator to something like "bin/start-metastore --config /stackable/config --db-type postgres --hive-bin-dir bin"
