# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

#
# Base image for builder stages in Java based products
#

FROM stackable/image/stackable-devel

ARG PRODUCT
ARG STACKABLE_USER_UID

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
  "temurin-${PRODUCT}-jdk" \
  krb5-devel \
  libcurl-devel \
  make \
  maven \
  openssl-devel \
  `# Required to unpack Omid tarball` \
  tar \
  wget \
  which \
  xz \
  zlib-devel
microdnf clean all
rm -rf /var/cache/yum
EOF

ENV JAVA_HOME="/usr/lib/jvm/temurin-${PRODUCT}-jdk"

COPY --chown=${STACKABLE_USER_UID}:0 java-devel/stackable/settings.xml /stackable/.m2/settings.xml
COPY --chown=${STACKABLE_USER_UID}:0 java-devel/stackable/settings.xml /root/.m2/settings.xml

# Mitigation for CVE-2021-44228 (Log4Shell)
# This variable is supported as of Log4j version 2.10 and
# disables the vulnerable feature
ENV LOG4J_FORMAT_MSG_NO_LOOKUPS=true
