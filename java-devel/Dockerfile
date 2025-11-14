# syntax=docker/dockerfile:1.20.0@sha256:26147acbda4f14c5add9946e2fd2ed543fc402884fd75146bd342a7f6271dc1d
# check=error=true

#
# Base image for builder stages in Java based products
#

FROM local-image/stackable-devel

ARG PRODUCT_VERSION
ARG STACKABLE_USER_UID

# Find the latest version here: https://github.com/apache/maven/releases
# renovate: datasource=github-tags packageName=apache/maven
ARG MAVEN_VERSION="3.9.11"

# See: https://adoptium.net/en-gb/installation/linux/#_centosrhelfedora_instructions
RUN cat <<EOF > /etc/yum.repos.d/adoptium.repo
[Adoptium]
[Adoptium]
name=Adoptium
# The adoptium mirror times-out often, so we have created a pull-through cache
# baseurl=https://packages.adoptium.net/artifactory/rpm/rhel/\$releasever/\$basearch
baseurl=https://build-repo.stackable.tech/repository/Adoptium/\$releasever/\$basearch
enabled=1
gpgcheck=1
gpgkey=https://packages.adoptium.net/artifactory/api/gpg/key/public
EOF

RUN <<EOF
microdnf update
microdnf install \
  cmake \
  cyrus-sasl-devel \
  `# diff is required by maven during the build of hbase` \
  `# Cannot run program "diff" (in directory "/stackable/hbase-2.4.12-src/hbase-shaded/hbase-shaded-check-invariants"` \
  diffutils \
  fuse-devel \
  gcc \
  gcc-c++ \
  `# The GNU gettext utilities contain the envsubst program which` \
  `# substitutes the values of environment variables.` \
  gettext \
  `# For the apply_patches.sh script`\
  git \
  `# Needed by the maven ant run plugin for the "set-hostname-property" step in zookeeper` \
  hostname \
  `# Needed for compiling Java projects` \
  "temurin-${PRODUCT_VERSION}-jdk" \
  krb5-devel \
  libcurl-devel \
  make \
  openssl-devel \
  `# Required to unpack Omid tarball` \
  tar \
  wget \
  which \
  xz \
  zlib-devel
microdnf clean all
rm -rf /var/cache/yum

curl "https://repo.stackable.tech/repository/packages/maven/apache-maven-${MAVEN_VERSION}-bin.tar.gz" | tar -xzC /tmp
mv /tmp/apache-maven-${MAVEN_VERSION} /opt/maven
ln -s /opt/maven/bin/mvn /usr/bin/mvn

EOF

ENV JAVA_HOME="/usr/lib/jvm/temurin-${PRODUCT_VERSION}-jdk"
ENV MAVEN_ARGS="--batch-mode --no-transfer-progress"

COPY --chown=${STACKABLE_USER_UID}:0 java-devel/stackable/settings.xml /stackable/.m2/settings.xml
COPY --chown=${STACKABLE_USER_UID}:0 java-devel/stackable/settings.xml /root/.m2/settings.xml

# Mitigation for CVE-2021-44228 (Log4Shell)
# This variable is supported as of Log4j version 2.10 and
# disables the vulnerable feature
ENV LOG4J_FORMAT_MSG_NO_LOOKUPS=true
