# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

FROM local-image/hadoop/hadoop AS hadoop-builder
FROM local-image/hive/hive-metastore-opa-authorizer AS hive-metastore-opa-authorizer-builder

FROM local-image/java-devel AS hive-builder


# Apache Hive up to 4.0.x(!) officially requires Java 8 (there is no distinction between building and running).
# As of 2024-04-15 we for sure need Java 8 for building, but we used a Java 11 runtime for months now without any problems.
# As we got weird TLS errors (https://stackable-workspace.slack.com/archives/C031A5BEFS7/p1713185172557459) with a
# Java 8 runtime we bumped the Runtime to Java 11 again.

ARG PRODUCT_VERSION
ARG RELEASE_VERSION
ARG HADOOP_HADOOP_VERSION
# Reassign the arg to `HADOOP_VERSION` for better readability.
ENV HADOOP_VERSION=${HADOOP_HADOOP_VERSION}
ARG JMX_EXPORTER_VERSION
ARG AWS_JAVA_SDK_BUNDLE_VERSION
ARG AZURE_STORAGE_VERSION
ARG AZURE_KEYVAULT_CORE_VERSION
ARG STACKABLE_USER_UID

# Setting this to anything other than "true" will keep the cache folders around (e.g. for Maven, NPM etc.)
# This can be used to speed up builds when disk space is of no concern.
ARG DELETE_CACHES="true"

# Copy patches into the builder
COPY --chown=${STACKABLE_USER_UID}:0 hive/stackable/patches/patchable.toml /stackable/src/hive/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 hive/stackable/patches/${PRODUCT_VERSION} /stackable/src/hive/stackable/patches/${PRODUCT_VERSION}
# It is useful to see which version of Hadoop is used at a glance
# Therefore the use of the full name here
# TODO: Do we really need all of Hadoop in here?
COPY --chown=${STACKABLE_USER_UID}:0 --from=hadoop-builder /stackable/hadoop-${HADOOP_VERSION}-stackable${RELEASE_VERSION} /stackable/hadoop-${HADOOP_VERSION}-stackable${RELEASE_VERSION}
COPY --chown=${STACKABLE_USER_UID}:0 --from=hadoop-builder /stackable/patched-libs /stackable/patched-libs

USER ${STACKABLE_USER_UID}
WORKDIR /stackable

ENV NEW_VERSION="${PRODUCT_VERSION}-stackable${RELEASE_VERSION}"

# Let's have patchable as a dedicated step, as it fetches the Hive sourcecode over the network,
# thus taking a bit (which is annoying while development)
RUN /stackable/patchable --images-repo-root=src checkout hive ${PRODUCT_VERSION} > /tmp/HIVE_SOURCE_DIR

# Use bash for regex machting, otherwise docker lint is complaining:
# hive/Dockerfile:51 SC3014 warning: In POSIX sh, == in place of = is undefined.
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

# Make expensive maven build a separate layer for better caching
# Cache mounts are owned by root by default
# We need to explicitly give the uid to use
RUN --mount=type=cache,id=maven-hive-${PRODUCT_VERSION},uid=${STACKABLE_USER_UID},target=/stackable/.m2/repository <<EOF
set -e

BUILD_SRC_DIR="$(cat /tmp/HIVE_SOURCE_DIR)" || exit 1
rm /tmp/HIVE_SOURCE_DIR
cd "$BUILD_SRC_DIR"

# Make Maven aware of custom Stackable libraries
cp -r /stackable/patched-libs/maven/* /stackable/.m2/repository

# generateBackupPoms=false is needed for the Hive 4.0.0 build to succeed, otherwise it fails with the obscure reason: `Too many files with unapproved license`
mvn versions:set -DnewVersion=$NEW_VERSION -DartifactId=* -DgroupId=* -DgenerateBackupPoms=false

# Create snapshot of the source code including custom patches
tar -czf /stackable/hive-${NEW_VERSION}-src.tar.gz .

if [[ "${PRODUCT_VERSION}" == "3.1.3" ]] ; then
  mvn \
   clean package \
   -DskipTests \
    --projects standalone-metastore
  mv standalone-metastore/target/apache-hive-metastore-${NEW_VERSION}-bin/apache-hive-metastore-${NEW_VERSION}-bin /stackable
  mv standalone-metastore/target/bom.json /stackable/apache-hive-metastore-${NEW_VERSION}-bin/apache-hive-metastore-${NEW_VERSION}.cdx.json
elif [[ "${PRODUCT_VERSION}" == 4.0.* ]]; then
  (
    # https://issues.apache.org/jira/browse/HIVE-20451 switched the metastore server packaging starting with 4.0.0
    mvn \
    clean package \
    -DskipTests \
    -Dhadoop.version=${HADOOP_VERSION}-stackable${RELEASE_VERSION}

    # We only seem to get a .tar.gz archive, so let's extract that to the correct location
    tar --extract --directory=/stackable -f standalone-metastore/metastore-server/target/apache-hive-standalone-metastore-server-${NEW_VERSION}-bin.tar.gz
    mv standalone-metastore/metastore-server/target/bom.json /stackable/apache-hive-metastore-${NEW_VERSION}-bin/apache-hive-metastore-${NEW_VERSION}.cdx.json
    # TODO: Remove once the fix https://github.com/apache/hive/pull/5419 is merged and released
    # The schemaTool.sh is still pointing to the class location from Hive < 4.0.0, it seems like it was forgotten to update it
    sed -i -e 's/CLASS=org.apache.hadoop.hive.metastore.tools.MetastoreSchemaTool/CLASS=org.apache.hadoop.hive.metastore.tools.schematool.MetastoreSchemaTool/' /stackable/apache-hive-metastore-${NEW_VERSION}-bin/bin/ext/schemaTool.sh
  )
else
  # Starting with 4.1.0 the build process changed again in https://github.com/apache/hive/pull/5936 (HIVE-29062)
  mvn \
  clean package \
  -Dhadoop.version=${HADOOP_VERSION}-stackable${RELEASE_VERSION} \
  -DskipTests \
  -Pdist
  # Looks like we can not filter the projects using "--projects standalone-metastore/metastore-server --also-make",
  # as this does not build a *.tar.gz

  # We only seem to get a .tar.gz archive, so let's extract that to the correct location
  tar --extract --directory=/stackable -f standalone-metastore/packaging/target/hive-standalone-metastore-${NEW_VERSION}-bin.tar.gz
  mv standalone-metastore/metastore-server/target/bom.json /stackable/apache-hive-metastore-${NEW_VERSION}-bin/hive-standalone-metastore-${NEW_VERSION}.cdx.json
fi

# Remove sourcecode
rm -rf "$BUILD_SRC_DIR"
EOF

RUN <<EOF
cd /stackable
mkdir /stackable/jmx
curl "https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER_VERSION}.jar" -o "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER_VERSION}.jar"
ln -s "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER_VERSION}.jar" /stackable/jmx/jmx_prometheus_javaagent.jar

# Needed to run housekeeping jobs, see footnote <1> below
cp /stackable/patched-libs/maven/org/apache/hadoop/hadoop-mapreduce-client-core/${HADOOP_VERSION}-stackable${RELEASE_VERSION}/hadoop-mapreduce-client-core-${HADOOP_VERSION}-stackable${RELEASE_VERSION}.jar /stackable/apache-hive-metastore-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-bin/lib/

# The next two sections for S3 and Azure use hardcoded version numbers on purpose instead of wildcards
# This way the build will fail should one of the files not be available anymore in a later Hadoop version!

# Add S3 Support for Hive (support for s3a://)
cp /stackable/hadoop-${HADOOP_VERSION}-stackable${RELEASE_VERSION}/share/hadoop/tools/lib/hadoop-aws-${HADOOP_VERSION}-stackable${RELEASE_VERSION}.jar /stackable/apache-hive-metastore-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-bin/lib/

# According to https://hadoop.apache.org/docs/stable/hadoop-aws/tools/hadoop-aws/aws_sdk_upgrade.html, the jar filename has changed from
# aws-java-sdk-bundle-${AWS_JAVA_SDK_BUNDLE_VERSION}.jar to bundle-${AWS_JAVA_SDK_BUNDLE_VERSION}.jar. In future, you might need to do:
if [[ "${PRODUCT_VERSION}" == "3.1.3" || "${PRODUCT_VERSION}" == 4.0.* ]]; then
cp /stackable/hadoop-${HADOOP_VERSION}-stackable${RELEASE_VERSION}/share/hadoop/tools/lib/aws-java-sdk-bundle-${AWS_JAVA_SDK_BUNDLE_VERSION}.jar /stackable/apache-hive-metastore-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-bin/lib/
else
cp /stackable/hadoop-${HADOOP_VERSION}-stackable${RELEASE_VERSION}/share/hadoop/tools/lib/bundle-${AWS_JAVA_SDK_BUNDLE_VERSION}.jar /stackable/apache-hive-metastore-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-bin/lib/
fi

# Add Azure ABFS support (support for abfs://)
cp /stackable/hadoop-${HADOOP_VERSION}-stackable${RELEASE_VERSION}/share/hadoop/tools/lib/hadoop-azure-${HADOOP_VERSION}-stackable${RELEASE_VERSION}.jar /stackable/apache-hive-metastore-${NEW_VERSION}-bin/lib/
cp /stackable/hadoop-${HADOOP_VERSION}-stackable${RELEASE_VERSION}/share/hadoop/tools/lib/azure-storage-${AZURE_STORAGE_VERSION}.jar /stackable/apache-hive-metastore-${NEW_VERSION}-bin/lib/
cp /stackable/hadoop-${HADOOP_VERSION}-stackable${RELEASE_VERSION}/share/hadoop/tools/lib/azure-keyvault-core-${AZURE_KEYVAULT_CORE_VERSION}.jar /stackable/apache-hive-metastore-${NEW_VERSION}-bin/lib/

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

FROM local-image/java-base AS final

ARG PRODUCT_VERSION
ARG HADOOP_HADOOP_VERSION
# Reassign the arg to `HADOOP_VERSION` for better readability.
ENV HADOOP_VERSION=${HADOOP_HADOOP_VERSION}
ARG RELEASE_VERSION
ARG STACKABLE_USER_UID


ARG NAME="Apache Hive metastore"
ARG DESCRIPTION="This image is deployed by the Stackable Operator for Apache Hive."

LABEL name="Apache Hive metastore"
LABEL version="${PRODUCT_VERSION}"
LABEL release="${RELEASE_VERSION}"
LABEL summary="The Stackable image for Apache Hive metastore."
LABEL description="${DESCRIPTION}"

# https://github.com/opencontainers/image-spec/blob/036563a4a268d7c08b51a08f05a02a0fe74c7268/annotations.md#annotations
LABEL org.opencontainers.image.documentation="https://docs.stackable.tech/home/stable/hive/"
LABEL org.opencontainers.image.version="${PRODUCT_VERSION}"
LABEL org.opencontainers.image.revision="${RELEASE_VERSION}"
LABEL org.opencontainers.image.title="${NAME}"
LABEL org.opencontainers.image.description="${DESCRIPTION}"

# https://docs.openshift.com/container-platform/4.16/openshift_images/create-images.html#defining-image-metadata
# https://github.com/projectatomic/ContainerApplicationGenericLabels/blob/master/vendor/redhat/labels.md
LABEL io.openshift.tags="ubi9,stackable,hive,sdp"
LABEL io.k8s.description="${DESCRIPTION}"
LABEL io.k8s.display-name="${NAME}"

WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 --from=hive-builder /stackable/apache-hive-metastore-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-bin /stackable/apache-hive-metastore-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-bin
COPY --chown=${STACKABLE_USER_UID}:0 --from=hive-builder /stackable/hive-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-src.tar.gz /stackable
COPY --chown=${STACKABLE_USER_UID}:0 --from=hive-builder /stackable/hadoop-${HADOOP_VERSION}-stackable${RELEASE_VERSION} /stackable/hadoop-${HADOOP_VERSION}-stackable${RELEASE_VERSION}
COPY --chown=${STACKABLE_USER_UID}:0 --from=hadoop-builder /stackable/*-src.tar.gz /stackable
COPY --chown=${STACKABLE_USER_UID}:0 --from=hive-builder /stackable/jmx /stackable/jmx
COPY --chown=${STACKABLE_USER_UID}:0 --from=hive-metastore-opa-authorizer-builder /stackable/opa-authorizer-bin /stackable/apache-hive-metastore-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-bin/lib
COPY --chown=${STACKABLE_USER_UID}:0 --from=hive-metastore-opa-authorizer-builder /stackable/opa-authorizer-src /stackable
COPY --chown=${STACKABLE_USER_UID}:0 hive/stackable/jmx /stackable/jmx
COPY --chown=${STACKABLE_USER_UID}:0 hive/stackable/bin/start-metastore /stackable/apache-hive-metastore-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-bin/bin

COPY hive/licenses /licenses

RUN <<EOF
microdnf update
microdnf clean all
rpm -qa --qf "%{NAME}-%{VERSION}-%{RELEASE_VERSION}\n" | sort > /stackable/package_manifest.txt
chown ${STACKABLE_USER_UID}:0 /stackable/package_manifest.txt
chmod g=u /stackable/package_manifest.txt
rm -rf /var/cache/yum

chmod g=u /stackable/apache-hive-metastore-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-bin/bin/start-metastore

ln -s /stackable/apache-hive-metastore-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-bin /stackable/hive-metastore
chown -h ${STACKABLE_USER_UID}:0 /stackable/hive-metastore
chmod g=u /stackable/hive-metastore
ln -s /stackable/hadoop-${HADOOP_VERSION}-stackable${RELEASE_VERSION} /stackable/hadoop
chown -h ${STACKABLE_USER_UID}:0 /stackable/hadoop
chmod g=u /stackable/hadoop
chmod g=u /stackable/*-src.tar.gz

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

ENV HIVE_HOME=/stackable/hive-metastore
ENV HADOOP_HOME=/stackable/hadoop
ENV PATH="${PATH}:${HADOOP_HOME}/bin:${HIVE_HOME}/bin"

# The following 2 env-vars are required for common hadoop scripts even if the respective libraries are never used.
# We set them here to a sensible default.
ENV HADOOP_YARN_HOME=/stackable/hadoop
ENV HADOOP_MAPRED_HOME=/stackable/hadoop

WORKDIR /stackable/hive-metastore
# Start command is set by operator to something like "bin/start-metastore --config /stackable/config --db-type postgres --hive-bin-dir bin"



# <1>: org.apache.hadoop.mapred.JobConf need
# 2025-10-06T08:42:04,137 ERROR [Metastore threads starter thread] metastore.HiveMetaStore: Failure when starting the leader tasks, Compaction or Housekeeping tasks may not happen
# java.lang.NoClassDefFoundError: org/apache/hadoop/mapred/JobConf
#         at org.apache.hadoop.hive.conf.HiveConf.initialize(HiveConf.java:6601) ~[hive-common-4.1.0.jar:4.1.0]
#         at org.apache.hadoop.hive.conf.HiveConf.<init>(HiveConf.java:6569) ~[hive-common-4.1.0.jar:4.1.0]
#         at org.apache.hadoop.hive.ql.txn.compactor.CompactorThread.setConf(CompactorThread.java:68) ~[hive-exec-4.1.0-core.jar:4.1.0]
#         at org.apache.hadoop.hive.metastore.leader.CompactorTasks.takeLeadership(CompactorTasks.java:139) ~[hive-standalone-metastore-server-4.1.0-stackable0.0.0-dev.jar:4.1.0-stackable0.0.0-dev]
#         at org.apache.hadoop.hive.metastore.leader.LeaseLeaderElection.lambda$notifyListener$0(LeaseLeaderElection.java:141) ~[hive-standalone-metastore-server-4.1.0-stackable0.0.0-dev.jar:4.1.0-stackable0.0.0-dev]
#         at java.base/java.util.ArrayList.forEach(Unknown Source) ~[?:?]
#         at org.apache.hadoop.hive.metastore.leader.LeaseLeaderElection.notifyListener(LeaseLeaderElection.java:138) ~[hive-standalone-metastore-server-4.1.0-stackable0.0.0-dev.jar:4.1.0-stackable0.0.0-dev]
#         at org.apache.hadoop.hive.metastore.leader.LeaseLeaderElection.doWork(LeaseLeaderElection.java:120) ~[hive-standalone-metastore-server-4.1.0-stackable0.0.0-dev.jar:4.1.0-stackable0.0.0-dev]
#         at org.apache.hadoop.hive.metastore.leader.LeaseLeaderElection.tryBeLeader(LeaseLeaderElection.java:181) ~[hive-standalone-metastore-server-4.1.0-stackable0.0.0-dev.jar:4.1.0-stackable0.0.0-dev]
#         at org.apache.hadoop.hive.metastore.leader.LeaseLeaderElection.tryBeLeader(LeaseLeaderElection.java:63) ~[hive-standalone-metastore-server-4.1.0-stackable0.0.0-dev.jar:4.1.0-stackable0.0.0-dev]
#         at org.apache.hadoop.hive.metastore.leader.LeaderElectionContext.lambda$start$2(LeaderElectionContext.java:125) ~[hive-standalone-metastore-server-4.1.0-stackable0.0.0-dev.jar:4.1.0-stackable0.0.0-dev]
#         at java.base/java.lang.Thread.run(Unknown Source) ~[?:?]
#         at org.apache.hadoop.hive.metastore.leader.LeaderElectionContext.start(LeaderElectionContext.java:136) ~[hive-standalone-metastore-server-4.1.0-stackable0.0.0-dev.jar:4.1.0-stackable0.0.0-dev]
#         at org.apache.hadoop.hive.metastore.HiveMetaStore$8.run(HiveMetaStore.java:856) [hive-standalone-metastore-server-4.1.0-stackable0.0.0-dev.jar:4.1.0-stackable0.0.0-dev]
# Caused by: java.lang.ClassNotFoundException: org.apache.hadoop.mapred.JobConf
#         at java.base/jdk.internal.loader.BuiltinClassLoader.loadClass(Unknown Source) ~[?:?]
#         at java.base/jdk.internal.loader.ClassLoaders$AppClassLoader.loadClass(Unknown Source) ~[?:?]
#         at java.base/java.lang.ClassLoader.loadClass(Unknown Source) ~[?:?]
#         ... 14 more
