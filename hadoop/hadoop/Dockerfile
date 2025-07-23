# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

FROM stackable/image/java-devel AS hadoop-builder

ARG PRODUCT
ARG RELEASE
ARG PROTOBUF
ARG STACKABLE_USER_UID

WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 shared/protobuf/stackable/patches/patchable.toml /stackable/src/shared/protobuf/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 shared/protobuf/stackable/patches/${PROTOBUF} /stackable/src/shared/protobuf/stackable/patches/${PROTOBUF}

RUN <<EOF
rpm --install --replacepkgs https://dl.fedoraproject.org/pub/epel/epel-release-latest-9.noarch.rpm
microdnf update
# boost is a build dependency starting in Hadoop 3.4.0 if compiling native code
# automake and libtool are required to build protobuf
microdnf install boost1.78-devel automake libtool
microdnf clean all
rm -rf /var/cache/yum
mkdir /opt/protobuf
chown ${STACKABLE_USER_UID}:0 /opt/protobuf
EOF

USER ${STACKABLE_USER_UID}
# This Protobuf version is the exact version as used in the Hadoop Dockerfile
# See https://github.com/apache/hadoop/blob/trunk/dev-support/docker/pkg-resolver/install-protobuf.sh
# (this was hardcoded in the Dockerfile in earlier versions of Hadoop, make sure to look at the exact version in Github)
RUN <<EOF
    cd "$(/stackable/patchable --images-repo-root=src checkout shared/protobuf ${PROTOBUF})"

    # Create snapshot of the source code including custom patches
    tar -czf /stackable/protobuf-${PROTOBUF}-src.tar.gz .

    ./autogen.sh
    ./configure --prefix=/opt/protobuf
    make "-j$(nproc)"
    make install
    (cd .. && rm -r ${PROTOBUF})
EOF

ENV PROTOBUF_HOME=/opt/protobuf
ENV PATH="${PATH}:/opt/protobuf/bin"

WORKDIR /build
COPY --chown=${STACKABLE_USER_UID}:0 hadoop/hadoop/stackable/patches/patchable.toml /build/src/hadoop/hadoop/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 hadoop/hadoop/stackable/patches/${PRODUCT} /build/src/hadoop/hadoop/stackable/patches/${PRODUCT}
COPY --chown=${STACKABLE_USER_UID}:0 hadoop/hadoop/stackable/fuse_dfs_wrapper /build
USER ${STACKABLE_USER_UID}
# Hadoop Pipes requires libtirpc to build, whose headers are not packaged in RedHat UBI, so skip building this module
# Build from source to enable FUSE module, and to apply custom patches.
# Also skip building the yarn, mapreduce and minicluster modules: this will result in the modules being excluded but not all
# jar files will be stripped if they are needed elsewhere e.g. share/hadoop/yarn will not be part of the build, but yarn jars
# will still exist in share/hadoop/tools as they would be needed by the resource estimator tool. Such jars are removed in a later step.
RUN <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout hadoop/hadoop ${PRODUCT})"

ORIGINAL_VERSION=$(mvn help:evaluate -Dexpression=project.version -q -DforceStdout)
NEW_VERSION=${PRODUCT}-stackable${RELEASE}

mvn versions:set -DnewVersion=${NEW_VERSION}

# Since we skip building the hadoop-pipes module, we need to set the version to the original version so it can be pulled from Maven Central
sed -e '/<artifactId>hadoop-pipes<\/artifactId>/,/<\/dependency>/ { s/<version>.*<\/version>/<version>'"$ORIGINAL_VERSION"'<\/version>/ }' -i hadoop-tools/hadoop-tools-dist/pom.xml

# Create snapshot of the source code including custom patches
tar -czf /stackable/hadoop-${NEW_VERSION}-src.tar.gz .

# We do not pass require.snappy because that is only built in to the MapReduce client and we don't need that
#
# Passing require.openssl SHOULD make the build fail if OpenSSL is not present.
# This does not work properly however because this builder image contains the openssl-devel package which creates a symlink from /usr/lib64/libcrypto.so to the real version.
# Therefore, this build does work but the final image does NOT contain the openssl-devel package which is why it fails there which is why we have to create the symlink over there manually.
# We still leave this flag in to automatically fail should anything with the packages or symlinks ever fail.
mvn \
    --batch-mode \
    --no-transfer-progress \
    clean package install \
    -Pdist,native \
    -pl '!hadoop-tools/hadoop-pipes' \
    -Dhadoop.version=${NEW_VERSION} \
    -Drequire.fuse=true \
    -Drequire.openssl=true \
    -DskipTests \
    -Dmaven.javadoc.skip=true

mkdir -p /stackable/patched-libs/maven/org/apache
cp -r /stackable/.m2/repository/org/apache/hadoop /stackable/patched-libs/maven/org/apache

rm -rf hadoop-dist/target/hadoop-${NEW_VERSION}/share/hadoop/yarn
rm -rf hadoop-dist/target/hadoop-${NEW_VERSION}/share/hadoop/mapreduce
rm hadoop-dist/target/hadoop-${NEW_VERSION}/share/hadoop/client/hadoop-client-minicluster-*.jar
rm hadoop-dist/target/hadoop-${NEW_VERSION}/share/hadoop/tools/lib/hadoop-minicluster-*.jar

cp -r hadoop-dist/target/hadoop-${NEW_VERSION} /stackable/hadoop-${NEW_VERSION}
sed -i "s/${NEW_VERSION}/${ORIGINAL_VERSION}/g" hadoop-dist/target/bom.json
mv hadoop-dist/target/bom.json /stackable/hadoop-${NEW_VERSION}/hadoop-${NEW_VERSION}.cdx.json

# HDFS fuse-dfs is not part of the regular dist output, so we need to copy it in ourselves
cp hadoop-hdfs-project/hadoop-hdfs-native-client/target/main/native/fuse-dfs/fuse_dfs /stackable/hadoop-${NEW_VERSION}/bin

# Remove source code
(cd .. && rm -r ${PRODUCT})

ln -s /stackable/hadoop-${NEW_VERSION} /stackable/hadoop

mv /build/fuse_dfs_wrapper /stackable/hadoop/bin

# Remove unneeded binaries:
#  - code sources
#  - mapreduce/yarn binaries that were built as cross-project dependencies
#  - minicluster (only used for testing) and test .jars
#  - json-io: this is a transitive dependency pulled in by cedarsoft/java-utils/json-io and is excluded in 3.4.0. See CVE-2023-34610.
rm -rf /stackable/hadoop/share/hadoop/common/sources/
rm -rf /stackable/hadoop/share/hadoop/hdfs/sources/
rm -rf /stackable/hadoop/share/hadoop/tools/sources/
rm -rf /stackable/hadoop/share/hadoop/tools/lib/json-io-*.jar
rm -rf /stackable/hadoop/share/hadoop/tools/lib/hadoop-mapreduce-client-*.jar
rm -rf /stackable/hadoop/share/hadoop/tools/lib/hadoop-yarn-server*.jar
find /stackable/hadoop -name 'hadoop-minicluster-*.jar' -type f -delete
find /stackable/hadoop -name 'hadoop-client-minicluster-*.jar' -type f -delete
find /stackable/hadoop -name 'hadoop-*tests.jar' -type f -delete
rm -rf /stackable/.m2

# Set correct groups; make sure only required artifacts for the final image are located in /stackable
chmod -R g=u /stackable
EOF
