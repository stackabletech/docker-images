# syntax=docker/dockerfile:1.8.1@sha256:e87caa74dcb7d46cd820352bfea12591f3dba3ddc4285e19c7dcd13359f7cefd

FROM stackable/image/hadoop AS hadoop-builder

FROM stackable/image/java-devel AS hbase-builder

ARG PRODUCT
ARG HBASE_THIRDPARTY
ARG HBASE_OPERATOR_TOOLS
ARG ASYNC_PROFILER
ARG PHOENIX
ARG HBASE_PROFILE
ARG JMX_EXPORTER
ARG HADOOP
ARG TARGETARCH
ARG TARGETOS

# Setting this to anything other than "true" will keep the cache folders around (e.g. for Maven, NPM etc.)
# This can be used to speed up builds when disk space is of no concern.
ARG DELETE_CACHES="true"

COPY hbase/licenses /licenses

USER stackable
WORKDIR /stackable

COPY --chown=stackable:stackable hbase/stackable/patches /stackable/patches
COPY --chown=stackable:stackable hbase/stackable/jmx/config${JMX_EXPORTER} /stackable/jmx

# Cache mounts are owned by root by default
# We need to explicitly give the uid to use which is hardcoded to "1000" in stackable-base
# And every cache needs its own id, we can't share them between stages because we might delete the caches
# at the end of a run while other stages are still using it.
# While this might work in theory it didn't in practice (FileNotFound exceptions etc.)

# The cache id has to include the product version that we are building because otherwise
# docker encounters race conditions when building multiple versions in parallel, as all
# builder containers will share the same cache and the `rm -rf` commands will fail
# with a "directory not empty" error on the first builder to finish, as other builders
# are still working in the cache directory.
RUN --mount=type=cache,id=maven-hbase-${PRODUCT},uid=1000,target=/stackable/.m2/repository <<EOF
###
### HBase
###
curl --fail -L "https://repo.stackable.tech/repository/packages/hbase/hbase-${PRODUCT}-src.tar.gz" | tar -xzC .
mv hbase-${PRODUCT} hbase-${PRODUCT}-src
cd /stackable/hbase-${PRODUCT}-src/
/stackable/patches/apply_patches.sh /stackable/patches/hbase/${PRODUCT}

# The release scripts of HBase also run the build twice (three times in fact, once again to build the site which we skip here).
# I chose to replicate that exact behavior for consistency so please don't merge the two mvn runs into one unless you really know what you're doing!
mvn --batch-mode --no-transfer-progress -Dhadoop.profile=3.0 -Dhadoop-three.version=${HADOOP} clean install -DskipTests
mvn --batch-mode --no-transfer-progress -Dhadoop.profile=3.0 -Dhadoop-three.version=${HADOOP} install assembly:single -DskipTests -Dcheckstyle.skip=true -Prelease
tar -xzf hbase-assembly/target/hbase-${PRODUCT}-bin.tar.gz -C /stackable/
rm -rf /stackable/hbase-${PRODUCT}-src
ln -s "/stackable/hbase-${PRODUCT}" /stackable/hbase

###
### JMX Prometheus Exporter/Agent
###
if [[ -n "${JMX_EXPORTER}" ]] ; then
  curl --fail -L "https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" -o "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar"
  chmod +x "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar"
  ln -s "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" /stackable/jmx/jmx_prometheus_javaagent.jar
fi

###
### Async Profiler
###
cd /stackable
export ARCH="${TARGETARCH/amd64/x64}"
curl --fail -L "https://repo.stackable.tech/repository/packages/async-profiler/async-profiler-${ASYNC_PROFILER}-${TARGETOS}-${ARCH}.tar.gz"  | tar -xzC .
ln -s "/stackable/async-profiler-${ASYNC_PROFILER}-${TARGETOS}-${ARCH}" /stackable/async-profiler

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

FROM stackable/image/java-devel AS opa-authorizer-builder

ARG OPA_AUTHORIZER
ARG DELETE_CACHES

USER stackable
WORKDIR /stackable

RUN --mount=type=cache,id=maven-opa,uid=1000,target=/stackable/.m2/repository <<EOF

###
### OPA Authorizer (only for 2.6 upwards)
###
if [[ -n "$OPA_AUTHORIZER" ]]; then
    git clone --depth 1 --branch "$OPA_AUTHORIZER" https://github.com/stackabletech/hbase-opa-authorizer.git
    mvn \
      --batch-mode \
      --no-transfer-progress \
      package \
      -DskipTests \
      -fhbase-opa-authorizer
else
  # Create a dummy jar to avoid errors when copying it the final image
  mkdir -p hbase-opa-authorizer/target
  touch hbase-opa-authorizer/target/hbase-opa-authorizer.jar
fi


if [ "${DELETE_CACHES}" = "true" ] ; then
  rm -rf /stackable/.m2/repository/*
fi
EOF

FROM stackable/image/java-devel AS hbase-operator-tools-builder

ARG HBASE_OPERATOR_TOOLS
ARG HBASE_THIRDPARTY
ARG PRODUCT

# Setting this to anything other than "true" will keep the cache folders around (e.g. for Maven, NPM etc.)
# This can be used to speed up builds when disk space is of no concern.
ARG DELETE_CACHES="true"

# Resolve paths in bin/hbck2
# The variable names are intentionally passed to envsubst in single-quotes,
# so that they are not expanded. Disabling ShellCheck rules in a Dockerfile
# does not work, so please ignore the according warning (SC2016).
COPY --chown=stackable:stackable hbase/stackable/bin/hbck2.env /stackable/bin/

USER stackable
WORKDIR /stackable

# Cache mounts are owned by root by default
# We need to explicitly give the uid to use which is hardcoded to "1000" in stackable-base
RUN --mount=type=cache,id=maven-hbase-operator-tools,uid=1000,target=/stackable/.m2/repository <<EOF

curl --fail -L "https://repo.stackable.tech/repository/packages/hbase-operator-tools/hbase-operator-tools-${HBASE_OPERATOR_TOOLS}-src.tar.gz" | tar -xzC .
mv hbase-operator-tools-${HBASE_OPERATOR_TOOLS} hbase-operator-tools-${HBASE_OPERATOR_TOOLS}-src
mvn \
  --batch-mode \
  --no-transfer-progress \
  -Dhbase.version=${PRODUCT} \
  -Dhbase-thirdparty.version=${HBASE_THIRDPARTY} \
  -DskipTests \
  -fhbase-operator-tools-${HBASE_OPERATOR_TOOLS}-src \
  package assembly:single

# We need the "*" here as the directory won't be the same as the final tar file for SNAPSHOTs which we currently have to use for 2.6
# And we're stripping the top level directory while extracting because it may be called different than the folder name when it's a SNAPSHOT
mkdir /stackable/hbase-operator-tools-${HBASE_OPERATOR_TOOLS}
tar -xz \
  -f hbase-operator-tools-${HBASE_OPERATOR_TOOLS}-src/hbase-operator-tools-assembly/target/hbase-operator-tools-*-bin.tar.gz \
  -C /stackable/hbase-operator-tools-${HBASE_OPERATOR_TOOLS}/ \
  --strip-components=1
rm -rf /stackable/hbase-operator-tools-${HBASE_OPERATOR_TOOLS}-src

envsubst '${PRODUCT}:${HBASE_OPERATOR_TOOLS}' < /stackable/bin/hbck2.env > /stackable/bin/hbck2
chmod +x /stackable/bin/hbck2
rm /stackable/bin/hbck2.env

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


# Splitting this out into its own builder so that Hadoop & HBase can be built in parallel
# envsubst is only available in java-devel which is why we don't just do this in the final image
FROM stackable/image/java-devel AS hadoop-s3-builder

ARG PRODUCT
ARG HADOOP

USER stackable
WORKDIR /stackable

COPY --from=hadoop-builder --chown=stackable:stackable \
    /stackable/hadoop/share/hadoop/tools/lib/aws-java-sdk-bundle-*.jar \
    /stackable/hadoop/share/hadoop/tools/lib/hadoop-aws-${HADOOP}.jar \
    /stackable/hadoop/share/hadoop/tools/lib/

COPY --chown=stackable:stackable hbase/stackable/bin/export-snapshot-to-s3.env /stackable/bin/

RUN <<EOF
# Resolve paths in bin/export-snapshot-to-s3
export LIBS=$(find /stackable/hadoop/share/hadoop -name '*.jar' -printf '%p:' | sed 's/:$//')

# The variable names are intentionally passed to envsubst in single-quotes,
# so that they are not expanded. Disabling ShellCheck rules in a Dockerfile
# does not work, so please ignore the according warning (SC2016).
envsubst '${PRODUCT}:${LIBS}' < /stackable/bin/export-snapshot-to-s3.env > /stackable/bin/export-snapshot-to-s3
chmod +x /stackable/bin/export-snapshot-to-s3
rm /stackable/bin/export-snapshot-to-s3.env
EOF

FROM stackable/image/java-devel AS phoenix-builder

ARG PRODUCT
ARG ASYNC_PROFILER
ARG PHOENIX
ARG HBASE_PROFILE
ARG HADOOP

# Setting this to anything other than "true" will keep the cache folders around (e.g. for Maven, NPM etc.)
# This can be used to speed up builds when disk space is of no concern.
ARG DELETE_CACHES="true"

USER stackable
WORKDIR /stackable

COPY --chown=stackable:stackable hbase/stackable/patches /stackable/patches

RUN --mount=type=cache,id=maven-phoenix,uid=1000,target=/stackable/.m2/repository <<EOF
cd /stackable
curl --fail -L "https://repo.stackable.tech/repository/packages/phoenix/phoenix-${PHOENIX}-src.tar.gz" | tar -xzC .
mv phoenix-${PHOENIX} phoenix-${PHOENIX}-src
cd /stackable/phoenix-${PHOENIX}-src/
if [ -d /stackable/patches/phoenix/${PHOENIX} ]; then
  /stackable/patches/apply_patches.sh /stackable/patches/phoenix/${PHOENIX}
fi

# The Maven command can be found inside of the scripts in the create-release folder (release-util.sh as of Phoenix 5.2.0)
# https://github.com/apache/phoenix/tree/5.2.0/dev/create-release
mvn \
  --batch-mode \
  --no-transfer-progress \
  -Dhbase.version=${PRODUCT} \
  -Dhbase.profile=${HBASE_PROFILE} \
  -DskipTests \
  -Dcheckstyle.skip=true \
  clean \
  package

# We need the "*" here as the directory won't be the same as the final tar file for SNAPSHOTs which we currently have to use for 2.6
# And we're stripping the top level directory while extracting because it may be called different than the folder name when it's a SNAPSHOT
mkdir /stackable/phoenix-${HBASE_PROFILE}-${PHOENIX}-bin
tar -xz -f phoenix-assembly/target/phoenix-hbase-*-bin.tar.gz -C /stackable/phoenix-${HBASE_PROFILE}-${PHOENIX}-bin/ --strip-components=1
ln -s "/stackable/phoenix-${HBASE_PROFILE}-${PHOENIX}-bin" /stackable/phoenix


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


# Final Image
FROM stackable/image/java-base AS final

ARG PRODUCT
ARG RELEASE
ARG HADOOP
ARG PHOENIX
ARG HBASE_PROFILE
ARG HBASE_OPERATOR_TOOLS

ARG NAME="Apache HBase"
ARG DESCRIPTION="This image is deployed by the Stackable Operator for Apache HBase"

LABEL name="${NAME}"
LABEL version="${PRODUCT}"
LABEL release="${RELEASE}"
LABEL summary="The Stackable image for Apache HBase"
LABEL description="${DESCRIPTION}"

# https://github.com/opencontainers/image-spec/blob/036563a4a268d7c08b51a08f05a02a0fe74c7268/annotations.md#annotations
LABEL org.opencontainers.image.documentation="https://docs.stackable.tech/home/stable/hbase/"
LABEL org.opencontainers.image.version="${PRODUCT}"
LABEL org.opencontainers.image.revision="${RELEASE}"
LABEL org.opencontainers.image.title="${NAME}"
LABEL org.opencontainers.image.description="${DESCRIPTION}"

# https://docs.openshift.com/container-platform/4.16/openshift_images/create-images.html#defining-image-metadata
# https://github.com/projectatomic/ContainerApplicationGenericLabels/blob/master/vendor/redhat/labels.md
LABEL io.openshift.tags="ubi9,stackable,hbase,sdp,nosql"
LABEL io.k8s.description="${DESCRIPTION}"
LABEL io.k8s.display-name="${NAME}"

COPY --chown=stackable:stackable --from=hbase-builder /stackable/hbase-${PRODUCT} /stackable/hbase-${PRODUCT}/
COPY --chown=stackable:stackable --from=hbase-builder /stackable/async-profiler /stackable/async-profiler/
COPY --chown=stackable:stackable --from=hbase-builder /stackable/jmx /stackable/jmx/

COPY --chown=stackable:stackable --from=hbase-operator-tools-builder /stackable/hbase-operator-tools-${HBASE_OPERATOR_TOOLS} /stackable/hbase-operator-tools-${HBASE_OPERATOR_TOOLS}/
COPY --chown=stackable:stackable --from=hbase-operator-tools-builder /stackable/bin/hbck2 /stackable/bin/hbck2

COPY --chown=stackable:stackable --from=phoenix-builder /stackable/phoenix /stackable/phoenix/

COPY --chown=stackable:stackable --from=hadoop-s3-builder /stackable/bin/export-snapshot-to-s3 /stackable/bin/export-snapshot-to-s3
COPY --chown=stackable:stackable --from=hadoop-s3-builder /stackable/hadoop/share/hadoop/tools/lib/ /stackable/hadoop/share/hadoop/tools/lib/

COPY --chown=stackable:stackable --from=opa-authorizer-builder /stackable/hbase-opa-authorizer/target/hbase-opa-authorizer*.jar /stackable/hbase-${PRODUCT}/lib

RUN <<EOF
microdnf update

# The tar and python packages are required by the Phoenix command line.
# We add zip and gzip because tar without compression is seldom useful.
microdnf install \
  gzip \
  python \
  python-pip \
  tar \
  zip

microdnf clean all
rpm -qa --qf "%{NAME}-%{VERSION}-%{RELEASE}\n" | sort > /stackable/package_manifest.txt
rm -rf /var/cache/yum

ln --symbolic --logical --verbose "/stackable/hbase-${PRODUCT}" /stackable/hbase
ln --symbolic --logical --verbose "/stackable/hbase-operator-tools-${HBASE_OPERATOR_TOOLS}" /stackable/hbase-operator-tools
ln --symbolic --logical --verbose "/stackable/phoenix/phoenix-server-hbase-${HBASE_PROFILE}.jar" "/stackable/hbase/lib/phoenix-server-hbase-${HBASE_PROFILE}.jar"
EOF

USER stackable
ENV HBASE_CONF_DIR=/stackable/hbase/conf
ENV HOME=/stackable
ENV PATH="${PATH}:/stackable/bin:/stackable/hbase/bin"
ENV ASYNC_PROFILER_HOME=/stackable/async-profiler

WORKDIR /stackable/hbase
CMD ["./bin/hbase", "master", "start" ]
