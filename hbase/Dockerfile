# syntax=docker/dockerfile:1.8.1@sha256:e87caa74dcb7d46cd820352bfea12591f3dba3ddc4285e19c7dcd13359f7cefd

# Ignoring DL3038 globally because set `assumeyes=True` in dnf.conf in our base image
# Ignoring DL4006 globally because we inherit the SHELL from our base image
# hadolint global ignore=DL3038,DL4006

FROM stackable/image/hadoop AS hadoop-builder

# hadolint ignore=DL3006
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

COPY hbase/licenses /licenses

USER stackable
WORKDIR /stackable

COPY --chown=stackable:stackable hbase/stackable/patches /stackable/patches
COPY --chown=stackable:stackable hbase/stackable/jmx /stackable/jmx

# Resolve paths in bin/hbck2
# The variable names are intentionally passed to envsubst in single-quotes,
# so that they are not expanded. Disabling ShellCheck rules in a Dockerfile
# does not work, so please ignore the according warning (SC2016).
COPY --chown=stackable:stackable hbase/stackable/bin/hbck2.env /stackable/bin/

COPY --from=hadoop-builder --chown=stackable:stackable \
    /stackable/hadoop/share/hadoop/tools/lib/aws-java-sdk-bundle-*.jar \
    /stackable/hadoop/share/hadoop/tools/lib/hadoop-aws-${HADOOP}.jar \
    /stackable/hadoop/share/hadoop/tools/lib/
COPY --chown=stackable:stackable hbase/stackable/bin/export-snapshot-to-s3.env /stackable/bin/

RUN <<EOF
###
### HBase
###
curl --fail -L "https://repo.stackable.tech/repository/packages/hbase/hbase-${PRODUCT}-src.tar.gz" | tar -xzC .
mv hbase-${PRODUCT} hbase-${PRODUCT}-src

chmod +x patches/apply_patches.sh
patches/apply_patches.sh ${PRODUCT}

cd /stackable/hbase-${PRODUCT}-src/

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
curl --fail -L "https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" -o "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar"
chmod +x "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar"
ln -s "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER}.jar" /stackable/jmx/jmx_prometheus_javaagent.jar

###
### Phoenix
###
cd /stackable
curl --fail -L "https://repo.stackable.tech/repository/packages/phoenix/phoenix-${PHOENIX}-src.tar.gz" | tar -xzC .
mv phoenix-${PHOENIX} phoenix-${PHOENIX}-src

# Passing "-f" means it'll build in the phoenix source directory without cding into it
mvn \
  --batch-mode \
  --no-transfer-progress \
  -Dhbase.version=${PRODUCT} \
  -Dhbase.profile=${HBASE_PROFILE} \
  -DskipTests \
  -fphoenix-${PHOENIX}-src \
  package

tar -xz -f phoenix-${PHOENIX}-src/phoenix-assembly/target/phoenix-hbase-${HBASE_PROFILE}-${PHOENIX}-bin.tar.gz -C /stackable/
rm -rf /stackable/phoenix-${PHOENIX}-src
ln -s "/stackable/phoenix-hbase-${HBASE_PROFILE}-${PHOENIX}-bin" /stackable/phoenix
ln -s "/stackable/phoenix/phoenix-server-hbase-${HBASE_PROFILE}-${PHOENIX}.jar" "/stackable/hbase/lib/phoenix-server-hbase-${HBASE_PROFILE}-${PHOENIX}.jar"

###
### Async Profiler
###
cd /stackable
export ARCH="${TARGETARCH/amd64/x64}"
curl --fail -L "https://repo.stackable.tech/repository/packages/async-profiler/async-profiler-${ASYNC_PROFILER}-${TARGETOS}-${ARCH}.tar.gz"  | tar -xzC .
ln -s "/stackable/async-profiler-${ASYNC_PROFILER}-${TARGETOS}-${ARCH}" /stackable/async-profiler

###
### HBase Operator Tools
###
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

tar -xz \
  -f hbase-operator-tools-${HBASE_OPERATOR_TOOLS}-src/hbase-operator-tools-assembly/target/hbase-operator-tools-${HBASE_OPERATOR_TOOLS}-bin.tar.gz \
  -C /stackable/
rm -rf /stackable/hbase-operator-tools-${HBASE_OPERATOR_TOOLS}-src

envsubst '${PRODUCT}:${HBASE_OPERATOR_TOOLS}' < bin/hbck2.env > /stackable/bin/hbck2
chmod +x /stackable/bin/hbck2
rm /stackable/bin/hbck2.env

# Resolve paths in bin/export-snapshot-to-s3
export LIBS=$(find /stackable/hadoop/share/hadoop -name '*.jar' -printf '%p:' | sed 's/:$//')

# The variable names are intentionally passed to envsubst in single-quotes,
# so that they are not expanded. Disabling ShellCheck rules in a Dockerfile
# does not work, so please ignore the according warning (SC2016).
envsubst '${PRODUCT}:${LIBS}' < /stackable/bin/export-snapshot-to-s3.env > /stackable/bin/export-snapshot-to-s3
chmod +x /stackable/bin/export-snapshot-to-s3
rm /stackable/bin/export-snapshot-to-s3.env
EOF

# Final Image
FROM stackable/image/java-base

ARG PRODUCT
ARG RELEASE
ARG HADOOP
ARG HBASE_OPERATOR_TOOLS

LABEL name="Apache HBase" \
    maintainer="info@stackable.tech" \
    vendor="Stackable GmbH" \
    version="${PRODUCT}" \
    release="${RELEASE}" \
    summary="The Stackable image for Apache HBase." \
    description="This image is deployed by the Stackable Operator for Apache HBase."

COPY --chown=stackable:stackable --from=hbase-builder /stackable/hbase-${PRODUCT} /stackable/hbase-${PRODUCT}/
COPY --chown=stackable:stackable --from=hbase-builder /stackable/hbase-operator-tools-${HBASE_OPERATOR_TOOLS} /stackable/hbase-operator-tools-${HBASE_OPERATOR_TOOLS}/
COPY --chown=stackable:stackable --from=hbase-builder /stackable/jmx /stackable/jmx/
COPY --chown=stackable:stackable --from=hbase-builder /stackable/phoenix /stackable/phoenix/
COPY --chown=stackable:stackable --from=hbase-builder /stackable/bin /stackable/bin/
COPY --chown=stackable:stackable --from=hbase-builder /stackable/hadoop /stackable/hadoop/
COPY --chown=stackable:stackable --from=hbase-builder /stackable/async-profiler /stackable/async-profiler/

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

ln -s /stackable/hbase-${PRODUCT} /stackable/hbase
ln -s /stackable/hbase-operator-tools-${HBASE_OPERATOR_TOOLS} /stackable/hbase-operator-tools
EOF

USER stackable
ENV HBASE_CONF_DIR=/stackable/hbase/conf
ENV HOME=/stackable
ENV PATH="${PATH}:/stackable/bin:/stackable/hbase/bin"
ENV ASYNC_PROFILER_HOME=/stackable/async-profiler

WORKDIR /stackable/hbase
CMD ["./bin/hbase", "master", "start" ]
