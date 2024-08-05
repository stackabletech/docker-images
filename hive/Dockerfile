# syntax=docker/dockerfile:1.8.1@sha256:e87caa74dcb7d46cd820352bfea12591f3dba3ddc4285e19c7dcd13359f7cefd

FROM stackable/image/hadoop AS hadoop-builder

FROM stackable/image/java-devel AS hive-builder

# Apache Hive up to 4.0.x(!) officially requires Java 8 (there is no distinction between building and running).
# As of 2024-04-15 we for sure need Java 8 for building, but we used a Java 11 runtime for months now without any problems.
# As we got weird TLS errors (https://stackable-workspace.slack.com/archives/C031A5BEFS7/p1713185172557459) with a
# Java 8 runtime we bumped the Runtime to Java 11 again.

ARG PRODUCT
ARG HADOOP
ARG JMX_EXPORTER

# Setting this to anything other than "true" will keep the cache folders around (e.g. for Maven, NPM etc.)
# This can be used to speed up builds when disk space is of no concern.
ARG DELETE_CACHES="true"

COPY --chown=stackable:stackable hive/stackable /stackable

USER stackable
WORKDIR /stackable

# Cache mounts are owned by root by default
# We need to explicitly give the uid to use which is hardcoded to "1000" in stackable-base
RUN --mount=type=cache,id=maven-hive,uid=1000,target=/stackable/.m2/repository <<EOF
curl --fail -L "https://repo.stackable.tech/repository/packages/hive/apache-hive-${PRODUCT}-src.tar.gz" | tar -xzC .

patches/apply_patches.sh ${PRODUCT}

cd /stackable/apache-hive-${PRODUCT}-src/
mvn --batch-mode --no-transfer-progress clean package -DskipTests --projects standalone-metastore
mv standalone-metastore/target/apache-hive-metastore-${PRODUCT}-bin/apache-hive-metastore-${PRODUCT}-bin /stackable

ln -s /stackable/apache-hive-metastore-${PRODUCT}-bin/ /stackable/hive-metastore
cp /stackable/bin/start-metastore /stackable/hive-metastore/bin
rm -rf /stackable/apache-hive-${PRODUCT}-src

curl --fail -L "https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" -o "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar"
ln -s "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" /stackable/jmx/jmx_prometheus_javaagent.jar

# We're removing these to make the intermediate layer smaller
# This can be necessary even though it's only a builder image because the GitHub Action Runners only have very limited space available
# and we are sometimes running into errors because we're out of space.
# Therefore, we try to clean up all layers as much as possible.
if [ "${DELETE_CACHES}" = "true" ] ; then
  rm -rf /stackable/.m2/repository/*
  rm -rf /stackable/.npm/*
  rm -rf /stackable/.cache/*
fi
EOF


FROM stackable/image/java-base AS final

ARG PRODUCT
ARG HADOOP
ARG RELEASE
ARG AWS_JAVA_SDK_BUNDLE
ARG AZURE_STORAGE
ARG AZURE_KEYVAULT_CORE


ARG NAME="Apache Hive"
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

RUN <<EOF
microdnf update
microdnf clean all
rpm -qa --qf "%{NAME}-%{VERSION}-%{RELEASE}\n" | sort > /stackable/package_manifest.txt
rm -rf /var/cache/yum
EOF

USER stackable
WORKDIR /stackable

COPY --chown=stackable:stackable --from=hive-builder /stackable/apache-hive-metastore-${PRODUCT}-bin /stackable/apache-hive-metastore-${PRODUCT}-bin
RUN ln -s /stackable/apache-hive-metastore-${PRODUCT}-bin /stackable/hive-metastore

# It is useful to see which version of Hadoop is used at a glance
# Therefore the use of the full name here
# TODO: Do we really need all of Hadoop in here?
COPY --chown=stackable:stackable --from=hadoop-builder /stackable/hadoop /stackable/hadoop-${HADOOP}
RUN ln -s /stackable/hadoop-${HADOOP} /stackable/hadoop

# The next two sections for S3 and Azure use hardcoded version numbers on purpose instead of wildcards
# This way the build will fail should one of the files not be available anymore in a later Hadoop version!

# Add S3 Support for Hive (support for s3a://)
RUN cp /stackable/hadoop/share/hadoop/tools/lib/hadoop-aws-${HADOOP}.jar /stackable/hive-metastore/lib/
RUN cp /stackable/hadoop/share/hadoop/tools/lib/aws-java-sdk-bundle-${AWS_JAVA_SDK_BUNDLE}.jar /stackable/hive-metastore/lib/

# Add Azure ABFS support (support for abfs://)
RUN cp /stackable/hadoop/share/hadoop/tools/lib/hadoop-azure-${HADOOP}.jar /stackable/hive-metastore/lib/
RUN cp /stackable/hadoop/share/hadoop/tools/lib/azure-storage-${AZURE_STORAGE}.jar /stackable/hive-metastore/lib/
RUN cp /stackable/hadoop/share/hadoop/tools/lib/azure-keyvault-core-${AZURE_KEYVAULT_CORE}.jar /stackable/hive-metastore/lib/

COPY --chown=stackable:stackable --from=hive-builder /stackable/jmx /stackable/jmx
COPY hive/licenses /licenses

ENV HADOOP_HOME=/stackable/hadoop
ENV HIVE_HOME=/stackable/hive-metastore
ENV PATH="${PATH}":/stackable/hadoop/bin:/stackable/hive-metastore/bin

# The following 2 env-vars are required for common hadoop scripts even if the respective libraries are never used.
# We set them here to a sensible default.
ENV HADOOP_YARN_HOME=/stackable/hadoop
ENV HADOOP_MAPRED_HOME=/stackable/hadoop

WORKDIR /stackable/hive-metastore
# Start command is set by operator to something like "bin/start-metastore --config /stackable/config --db-type postgres --hive-bin-dir bin"
