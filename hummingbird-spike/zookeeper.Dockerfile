# SPIKE: Apache ZooKeeper on Red Hat Hummingbird, without any UBI9 stage.
#
# UNTESTED. Self-contained replacement for the chain
#   zookeeper -> java-base -> vector -> stackable-base -> ubi9-minimal
#   zookeeper (builder) -> java-devel -> stackable-devel -> ubi9-minimal
# Local images (local-image/...) are replaced by named stages in this file.
#
# Build from the repository root (the build context must be the repo root):
#
#   podman build -f hummingbird-spike/zookeeper.Dockerfile -t zookeeper-hum:spike .
#
# Useful intermediate targets: devel, java-devel, rust-binaries, vector-builder, zookeeper-builder, rootfs
#
#   podman build -f hummingbird-spike/zookeeper.Dockerfile --target rootfs -t zk-hum-rootfs .
#
# Not covered yet (see hummingbird-findings.md):
# - nano / iputils (not in the Hummingbird repo)
# - inotify-tools is not in the Hummingbird repo either. As in vector/Dockerfile we install the
#   RPM from the Stackable package repo, but that is an el9 build on a Fedora-derived system.
# - arm64 (only amd64 was inspected)
# - SBOM (CycloneDX) for the JRE: the portable JRE is a tarball and does not appear in the rpmdb

ARG DEVEL_BASE=registry.access.redhat.com/hi/openjdk:21-builder
# TODO: pin the base by digest, like the other Dockerfiles in this repo do

ARG PRODUCT_VERSION=3.9.6
ARG RELEASE_VERSION=0.0.0-dev
ARG STACKABLE_USER_NAME=stackable
ARG STACKABLE_USER_UID=1000
ARG STACKABLE_USER_GID=1000
ARG JMX_EXPORTER_VERSION=1.4.0
ARG LOGBACK_VERSION=1.3.15
ARG JVMKILL_VERSION=0e410d8
ARG CONFIG_UTILS_VERSION=0.5.1
ARG CONTAINERDEBUG_VERSION=0.5.0
ARG CERT_TOOLS_VERSION=0.1.1
# From vector/boil-config.toml and stackable-devel/boil-config.toml (the "vector-build" variant)
ARG VECTOR_VERSION=0.58.0
ARG VECTOR_RUST_TOOLCHAIN_VERSION=1.95.0
ARG PROTOC_VERSION=36.2
ARG INOTIFY_TOOLS=3.22.1.0-1.el9

# ==============================================================================
# devel: replaces stackable-devel. Hummingbird builder variant + user + Rust + patchable
# NOTE: this base also ships a JDK 21 that we do not use (JAVA_HOME is overridden below).

FROM ${DEVEL_BASE} AS devel

ARG STACKABLE_USER_NAME
ARG STACKABLE_USER_UID
ARG STACKABLE_USER_GID

# Build stage only, nothing from it is shipped as is. The final image is FROM scratch and
# runs as ${STACKABLE_USER_UID}. root is needed because the base image defaults to UID 65532.
# hadolint ignore=DL3002
USER 0
SHELL ["/bin/bash", "-euo", "pipefail", "-c"]
# The base image sets HOME=/home/build
ENV HOME=/root

COPY stackable-base/stackable/dnf.conf /etc/dnf/dnf.conf
COPY stackable-base/stackable/curlrc /root/.curlrc

# Copied from stackable-devel/Dockerfile
ARG RUST_DEFAULT_TOOLCHAIN_VERSION=1.99.0
ENV RUST_DEFAULT_TOOLCHAIN_VERSION=${RUST_DEFAULT_TOOLCHAIN_VERSION}
ENV CARGO_CYCLONEDX_CRATE_VERSION=0.5.9
ENV CARGO_AUDITABLE_CRATE_VERSION=0.7.7

# tar, xz and gzip are not part of the builder image
# pkgconf-pkg-config provides the `pkg-config` command (package name unverified)
RUN <<EOF
dnf install \
  curl diffutils findutils gcc gettext git gzip hostname make openssl-devel \
  pkgconf pkgconf-pkg-config shadow-utils tar which xz
dnf clean all

groupadd --gid ${STACKABLE_USER_GID} --system ${STACKABLE_USER_NAME}
useradd \
  --no-log-init \
  --gid ${STACKABLE_USER_GID} \
  --uid ${STACKABLE_USER_UID} \
  --system \
  --create-home \
  --home-dir /stackable \
  ${STACKABLE_USER_NAME}

cp /root/.curlrc /stackable/.curlrc
chown ${STACKABLE_USER_UID}:0 /stackable/.curlrc

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain "$RUST_DEFAULT_TOOLCHAIN_VERSION"
. "$HOME/.cargo/env"
cargo --locked --quiet install cargo-cyclonedx@"$CARGO_CYCLONEDX_CRATE_VERSION" cargo-auditable@"$CARGO_AUDITABLE_CRATE_VERSION"
EOF

# patchable: tool for patch management
COPY rust/patchable/ /patchable/rust/patchable
COPY Cargo.* /patchable/
RUN <<EOF
cd /patchable
. "$HOME/.cargo/env"
cargo auditable --quiet build --release
mv /patchable/target/release/patchable /stackable/patchable
chown ${STACKABLE_USER_UID}:0 /stackable/patchable
rm -rf /patchable
EOF

# ==============================================================================
# java-devel: replaces java-devel. JDK 17 from the Hummingbird repo + Maven

FROM devel AS java-devel

# SHELL is not persisted in the OCI image format (podman's default), so a stage that is
# based on another stage does not inherit it. Without this, RUN would use plain `sh -c`
# without `-e`, and a failing command in a heredoc would not fail the build.
SHELL ["/bin/bash", "-euo", "pipefail", "-c"]

ARG STACKABLE_USER_UID

# Copied from java-devel/Dockerfile
ARG MAVEN_VERSION=3.10.0
ENV VERSIONS_MAVEN_PLUGIN_VERSION=2.23.0
ENV HELP_MAVEN_PLUGIN_VERSION=3.5.2
ENV DEPENDENCY_MAVEN_PLUGIN_VERSION=3.11.0
ENV MAVEN_ARGS="--batch-mode --no-transfer-progress"
ENV LOG4J_FORMAT_MSG_NO_LOOKUPS=true

# The "portable" JDK RPMs only ship a tarball in /usr/lib/jvm, so we unpack it ourselves.
# The tarball name pattern (*portable.jdk*) is unverified.
RUN <<EOF
dnf install java-17-openjdk-portable-devel
dnf clean all
JDK_TARBALL="$(ls /usr/lib/jvm/*portable.jdk*.tar.xz)"
mkdir -p /usr/lib/jvm/java-17
tar -xJf "$JDK_TARBALL" -C /usr/lib/jvm/java-17 --strip-components=1

curl "https://repo.stackable.tech/repository/packages/maven/apache-maven-${MAVEN_VERSION}-bin.tar.gz" | tar -xzC /tmp
mv /tmp/apache-maven-${MAVEN_VERSION} /opt/maven
ln -s /opt/maven/bin/mvn /usr/bin/mvn
EOF

ENV JAVA_HOME=/usr/lib/jvm/java-17
ENV PATH="/usr/lib/jvm/java-17/bin:${PATH}"

COPY --chown=${STACKABLE_USER_UID}:0 java-devel/stackable/settings.xml /stackable/.m2/settings.xml
COPY --chown=${STACKABLE_USER_UID}:0 java-devel/stackable/settings.xml /root/.m2/settings.xml

# ==============================================================================
# patched-logback: replaces shared/logback

FROM java-devel AS patched-logback

SHELL ["/bin/bash", "-euo", "pipefail", "-c"]

ARG LOGBACK_VERSION
ARG STACKABLE_USER_UID

COPY --chown=${STACKABLE_USER_UID}:0 shared/logback/stackable/patches/patchable.toml /stackable/src/shared/logback/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 shared/logback/stackable/patches/${LOGBACK_VERSION} /stackable/src/shared/logback/stackable/patches/${LOGBACK_VERSION}

USER ${STACKABLE_USER_UID}
ENV HOME=/stackable
WORKDIR /stackable

RUN <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout shared/logback ${LOGBACK_VERSION})"
tar -czf /stackable/logback-${LOGBACK_VERSION}-src.tar.gz .
mvn --batch-mode --no-transfer-progress clean install -DskipTests -Dmaven.test.skip=true -Dmaven.javadoc.skip=true
EOF

# ==============================================================================
# jvmkill: replaces java-base/jvmkill (built against JDK 17 headers)

FROM java-devel AS jvmkill-builder

SHELL ["/bin/bash", "-euo", "pipefail", "-c"]

ARG JVMKILL_VERSION
WORKDIR /stackable

COPY --chown=0:0 java-base/jvmkill/stackable/patches/patchable.toml /stackable/src/java-base/jvmkill/stackable/patches/patchable.toml
COPY --chown=0:0 java-base/jvmkill/stackable/patches/${JVMKILL_VERSION} /stackable/src/java-base/jvmkill/stackable/patches/${JVMKILL_VERSION}

RUN <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout java-base/jvmkill ${JVMKILL_VERSION})"
make
cp libjvmkill.so /stackable/libjvmkill.so
cd /stackable
rm -rf ./src
EOF

# ==============================================================================
# rust-binaries: config-utils, containerdebug, cert-tools (copied from stackable-base/Dockerfile)

FROM devel AS rust-binaries

SHELL ["/bin/bash", "-euo", "pipefail", "-c"]

ARG CONFIG_UTILS_VERSION
ARG CONTAINERDEBUG_VERSION
ARG CERT_TOOLS_VERSION

RUN <<EOF
. "$HOME/.cargo/env"

cd /
git clone --depth 1 --branch "config-utils-${CONFIG_UTILS_VERSION}" https://github.com/stackabletech/operator-rs config-utils
git clone --depth 1 --branch "containerdebug-${CONTAINERDEBUG_VERSION}" https://github.com/stackabletech/operator-rs containerdebug
git clone --depth 1 --branch "cert-tools-${CERT_TOOLS_VERSION}" https://github.com/stackabletech/secret-operator
cd /config-utils
cargo auditable --quiet build --package config-utils --release && cargo cyclonedx --all --spec-version 1.5 --describe binaries --no-build-deps
cd /containerdebug
cargo auditable --quiet build --package containerdebug --release && cargo cyclonedx --all --spec-version 1.5 --describe binaries --no-build-deps
cd /secret-operator
cargo auditable --quiet build --release --package cert-tools && cargo cyclonedx --all --spec-version 1.5 --describe binaries --no-build-deps
EOF

# ==============================================================================
# vector-builder: copied from vector/Dockerfile. PRODUCT_VERSION is called VECTOR_VERSION here
# because PRODUCT_VERSION is the ZooKeeper version in this file.

FROM devel AS vector-builder

SHELL ["/bin/bash", "-euo", "pipefail", "-c"]

ARG VECTOR_VERSION
ARG VECTOR_RUST_TOOLCHAIN_VERSION
ARG RELEASE_VERSION
ARG STACKABLE_USER_UID
ARG PROTOC_VERSION

# The packages vector/Dockerfile installs, plus unzip for protoc.
# `perl` is perl-interpreter here: the Hummingbird `perl` metapackage requires ~50 perl-* modules
# that are not in the repo (dnf: "nothing provides perl-IPC-Cmd ..."). Vector's openssl-sys
# builds a vendored OpenSSL from source by default, and its Configure script needs perl modules
# (FindBin, IPC-Cmd, ...) that we do not install, so we link the system OpenSSL instead (see
# OPENSSL_NO_VENDOR below). openssl-libs must then be in the runtime rootfs.
ENV OPENSSL_NO_VENDOR=1
RUN <<EOF
dnf install \
    cmake \
    automake \
    perl-interpreter \
    findutils \
    openssl-devel \
    pkgconf \
    pkgconf-pkg-config \
    tar \
    cyrus-sasl-devel \
    clang \
    unzip \
    `# the configure script of tikv-jemalloc-sys needs awk, which the builder image does not have` \
    gawk
dnf clean all
EOF

WORKDIR /opt/protoc
RUN ARCH=$(arch | sed 's/^aarch64$/aarch_64/') \
  && curl --fail --location --output protoc.zip "https://repo.stackable.tech/repository/packages/protoc/protoc-${PROTOC_VERSION}-linux-${ARCH}.zip" \
  && unzip protoc.zip \
  && rm protoc.zip
ENV PROTOC=/opt/protoc/bin/protoc

WORKDIR /stackable

COPY --chown=${STACKABLE_USER_UID}:0 vector/stackable/patches/patchable.toml /stackable/src/vector/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 vector/stackable/patches/${VECTOR_VERSION} /stackable/src/vector/stackable/patches/${VECTOR_VERSION}

RUN mkdir /app
COPY vector/copy_artifacts.sh /

RUN <<EOF
cd "$(/stackable/patchable --images-repo-root=src checkout vector ${VECTOR_VERSION})"

NEW_VERSION="${VECTOR_VERSION}-stackable${RELEASE_VERSION}"
tar -czf /stackable/vector-${NEW_VERSION}-src.tar.gz .

. "$HOME/.cargo/env"
# The devel stage has a newer default toolchain, vector uses what upstream pins
rustup default "${VECTOR_RUST_TOOLCHAIN_VERSION}"

# Keep in sync with vector/Dockerfile (single line, cargo-cyclonedx does not split on newlines)
VECTOR_FEATURES="sources-file,sources-internal_logs,transforms-remap,transforms-filter,transforms-route,sinks-vector,sinks-opentelemetry,sinks-console,sinks-blackhole,api,tikv-jemallocator"

cargo auditable --quiet build --locked --release --no-default-features --features "${VECTOR_FEATURES}"

cargo cyclonedx --all --spec-version 1.5 --describe binaries \
  --no-default-features --features "${VECTOR_FEATURES}" --no-build-deps

find target/release \
  -regextype egrep \
  -maxdepth 1 \
  -executable \
  -type f \
  ! -regex ".*\-[a-fA-F0-9]{16,16}$" \
  -exec /copy_artifacts.sh {} \;

echo "The following files will be copied to the runtime image: $(ls /app)"

chmod -R g=u /stackable
EOF

# ==============================================================================
# zookeeper-builder: copied from zookeeper/Dockerfile (only the FROM lines differ)

FROM java-devel AS zookeeper-builder

SHELL ["/bin/bash", "-euo", "pipefail", "-c"]

ARG PRODUCT_VERSION
ARG JMX_EXPORTER_VERSION
ARG STACKABLE_USER_UID
ARG RELEASE_VERSION

COPY --chown=${STACKABLE_USER_UID}:0 zookeeper/stackable/patches/patchable.toml /stackable/src/zookeeper/stackable/patches/patchable.toml
COPY --chown=${STACKABLE_USER_UID}:0 zookeeper/stackable/patches/${PRODUCT_VERSION} /stackable/src/zookeeper/stackable/patches/${PRODUCT_VERSION}
COPY --chown=${STACKABLE_USER_UID}:0 zookeeper/stackable/jmx /stackable/jmx
COPY --chown=${STACKABLE_USER_UID}:0 --from=patched-logback /stackable/.m2/repository /stackable/patched-logback-libs

USER ${STACKABLE_USER_UID}
ENV HOME=/stackable
WORKDIR /stackable

RUN <<EOF
mkdir -p /stackable/.m2/repository
cp -r /stackable/patched-logback-libs/* /stackable/.m2/repository

cd "$(/stackable/patchable --images-repo-root=src checkout zookeeper ${PRODUCT_VERSION})"

ORIGINAL_VERSION=$(mvn "org.apache.maven.plugins:maven-help-plugin:${HELP_MAVEN_PLUGIN_VERSION}:evaluate" -Dexpression=project.version -q -DforceStdout)
NEW_VERSION="${PRODUCT_VERSION}-stackable${RELEASE_VERSION}"

mvn "org.codehaus.mojo:versions-maven-plugin:${VERSIONS_MAVEN_PLUGIN_VERSION}:set" -DnewVersion=$NEW_VERSION

tar -czf /stackable/zookeeper-${NEW_VERSION}-src.tar.gz .

mvn \
  -pl "!zookeeper-client/zookeeper-client-c" \
  clean install \
  -DskipTests \
  -Pfull-build

mv zookeeper-assembly/target/apache-zookeeper-${NEW_VERSION}-bin.tar.gz /stackable
tar -C /stackable -xvzf /stackable/apache-zookeeper-${NEW_VERSION}-bin.tar.gz
sed -i "s/${NEW_VERSION}/${ORIGINAL_VERSION}/g" zookeeper-assembly/target/bom.json
mv zookeeper-assembly/target/bom.json /stackable/apache-zookeeper-${NEW_VERSION}-bin/apache-zookeeper-${NEW_VERSION}.cdx.json
rm -rf /stackable/apache-zookeeper-${NEW_VERSION}-bin/docs
rm /stackable/apache-zookeeper-${NEW_VERSION}-bin/README_packaging.md

curl "https://repo.stackable.tech/repository/packages/jmx-exporter/jmx_prometheus_javaagent-${JMX_EXPORTER_VERSION}.jar" \
  -o "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER_VERSION}.jar"
chmod +x "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER_VERSION}.jar"
ln -s "/stackable/jmx/jmx_prometheus_javaagent-${JMX_EXPORTER_VERSION}.jar" /stackable/jmx/jmx_prometheus_javaagent.jar

chmod -R g=u /stackable
EOF

# ==============================================================================
# rootfs: assembles the runtime filesystem from the Hummingbird RPM repo into /rootfs
# (replaces stackable-base + java-base + the final RUN steps of zookeeper).
# The final image is FROM scratch because the distroless runtime has no shell to run
# any of these steps in.

FROM ${DEVEL_BASE} AS rootfs

ARG PRODUCT_VERSION
ARG RELEASE_VERSION
ARG STACKABLE_USER_NAME
ARG STACKABLE_USER_UID
ARG STACKABLE_USER_GID
ARG TARGETARCH
ARG INOTIFY_TOOLS

# Build stage only, nothing from it is shipped as is. The final image is FROM scratch and
# runs as ${STACKABLE_USER_UID}. root is needed because the base image defaults to UID 65532.
# hadolint ignore=DL3002
USER 0
SHELL ["/bin/bash", "-euo", "pipefail", "-c"]
ENV HOME=/root

COPY stackable-base/stackable/dnf.conf /etc/dnf/dnf.conf

# Package list is a first guess based on what stackable-base, java-base and zkServer.sh use.
# grep/sed/gawk are unverified (zkServer.sh and friends are bash scripts).
# The JRE is installed on the build host only; we extract its tarball into the rootfs.
RUN <<EOF
dnf install tar xz
dnf install java-17-openjdk-portable

mkdir /rootfs
dnf --installroot=/rootfs --use-host-config install \
  bash coreutils curl findutils gawk grep krb5-workstation less sed tar ca-certificates openssl-libs shadow-utils

# Unpack the JRE (a tarball inside the RPM)
JRE_TARBALL="$(ls /usr/lib/jvm/*portable.jre*.tar.xz)"
mkdir -p /rootfs/usr/lib/jvm/java-17
tar -xJf "$JRE_TARBALL" -C /rootfs/usr/lib/jvm/java-17 --strip-components=1

# Stackable user and group (shadow-utils is removed again below)
chroot /rootfs groupadd --gid ${STACKABLE_USER_GID} --system ${STACKABLE_USER_NAME}
chroot /rootfs useradd \
  --no-log-init \
  --gid ${STACKABLE_USER_GID} \
  --uid ${STACKABLE_USER_UID} \
  --system \
  --create-home \
  --home-dir /stackable \
  ${STACKABLE_USER_NAME}
dnf --installroot=/rootfs --use-host-config remove shadow-utils
# Cleans the build host's cache. The installroot's own cache is deleted further below.
dnf clean all
EOF

# The vector sidecar waits for a shutdown file using `inotifywait` (see vector/Dockerfile).
# inotify-tools is not in the Hummingbird repo, so, as in vector/Dockerfile, we install the RPM
# from the Stackable package repo. It is built for el9 (glibc 2.34); the rootfs has glibc 2.43.
# rpm resolves its dependencies against the rootfs rpmdb, which is why this runs before the copies below.
# Hummingbird's rpm is configured with %_pkgverify_level=all, so a package signed with a key that
# is not in the rpmdb is rejected ("NOKEY ... does not verify"). On UBI9 that was only a warning.
# The package is from EPEL 9, so we import the EPEL 9 key into the rootfs rpmdb first.
# rpm fetches the URL itself, no curl needed.
# rpm 6 names the imported key after its full fingerprint, so the query below also pins the key
# (this is the fingerprint the package signature reports; compare it with the one Fedora publishes).
RUN <<EOF
rpm --root /rootfs --import https://dl.fedoraproject.org/pub/epel/RPM-GPG-KEY-EPEL-9
rpm --root /rootfs -q gpg-pubkey-ff8ad1344597106ece813b918a3872bf3228467c

ARCH="${TARGETARCH/amd64/x86_64}"
ARCH="${ARCH/arm64/aarch64}"
rpm --root /rootfs --install \
    "https://repo.stackable.tech/repository/packages/inotify-tools/inotify-tools-${INOTIFY_TOOLS}.${ARCH}.rpm"
EOF

# CVE-2023-37920: the Hummingbird ca-certificates package still contains the e-Tugra roots
COPY stackable-base/stackable/ca-cert-blocklist/ /rootfs/etc/pki/ca-trust/source/blocklist/
RUN <<EOF
chroot /rootfs update-ca-trust
if [ "$(chroot /rootfs trust list --filter=ca-anchors | grep -c 'E-Tugra')" != "0" ]; then
  echo "Still found E-Tugra root certificates, this should not happen!"
  exit 1;
fi
EOF

# Shell usage "nicer", as in stackable-base
# echo won't expand escape sequences. Consider printf.
# https://github.com/koalaman/shellcheck/wiki/SC2028
# Reason: This is complaining about the complicated PS1 statement, it is the same as in stackable-base.
# hadolint ignore=SC2028
RUN <<EOF
{
  echo "alias ll='ls -alF --color=auto'"
  echo "alias ls='ls --color=auto'"
  echo "alias ..='cd ..'"
  echo "export PS1='\u@\[\e[36m\]\H\[\e[m\] \[\e[32m\]\$(pwd)\[\e[m\] \\$ '"
} >> /rootfs/stackable/.bashrc
printf 'if [ -f ~/.bashrc ]; then\n\tsource ~/.bashrc\nfi\n' >> /rootfs/stackable/.profile
EOF

COPY stackable-base/stackable/curlrc /rootfs/stackable/.curlrc

# Rust utilities, jvmkill, licenses, permission check script
COPY --chown=${STACKABLE_USER_UID}:0 --from=rust-binaries /config-utils/target/release/config-utils /rootfs/stackable/config-utils
COPY --chown=${STACKABLE_USER_UID}:0 --from=rust-binaries /config-utils/crates/config-utils/config-utils_bin.cdx.xml /rootfs/stackable/config-utils.cdx.xml
COPY --chown=${STACKABLE_USER_UID}:0 --from=rust-binaries /containerdebug/target/release/containerdebug /rootfs/stackable/containerdebug
COPY --chown=${STACKABLE_USER_UID}:0 --from=rust-binaries /containerdebug/crates/containerdebug/containerdebug_bin.cdx.xml /rootfs/stackable/containerdebug_bin.cdx.xml
COPY --chown=${STACKABLE_USER_UID}:0 --from=rust-binaries /secret-operator/target/release/cert-tools /rootfs/stackable/cert-tools
COPY --chown=${STACKABLE_USER_UID}:0 --from=rust-binaries /secret-operator/rust/cert-tools/cert-tools_bin.cdx.xml /rootfs/stackable/cert-tools_bin.cdx.xml
COPY --chown=${STACKABLE_USER_UID}:0 --from=jvmkill-builder /stackable/libjvmkill.so /rootfs/stackable/libjvmkill.so
COPY --chown=${STACKABLE_USER_UID}:0 shared/checks/check-permissions-ownership.sh /rootfs/usr/bin/check-permissions-ownership.sh

# vector (same layering order for /licenses as the original chain: vector, java-base, zookeeper)
COPY --chown=${STACKABLE_USER_UID}:0 vector/licenses /rootfs/licenses
COPY --chown=${STACKABLE_USER_UID}:0 vector/stackable/bin /rootfs/stackable/bin
COPY --chown=${STACKABLE_USER_UID}:0 --from=vector-builder /app/* /rootfs/usr/local/bin/
COPY java-base/licenses /rootfs/licenses
COPY zookeeper/licenses /rootfs/licenses

# ZooKeeper
COPY --chown=${STACKABLE_USER_UID}:0 --from=zookeeper-builder /stackable/apache-zookeeper-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-bin /rootfs/stackable/apache-zookeeper-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-bin/
COPY --chown=${STACKABLE_USER_UID}:0 --from=zookeeper-builder /stackable/zookeeper-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-src.tar.gz /rootfs/stackable/
COPY --chown=${STACKABLE_USER_UID}:0 --from=zookeeper-builder /stackable/jmx /rootfs/stackable/jmx/
COPY --chown=${STACKABLE_USER_UID}:0 --from=patched-logback /stackable/*-src.tar.gz /rootfs/stackable/

RUN <<EOF
ln -s /stackable/apache-zookeeper-${PRODUCT_VERSION}-stackable${RELEASE_VERSION}-bin/ /rootfs/stackable/zookeeper

# The JRE is not in the rootfs rpmdb because it comes from a tarball, so add it by hand
{
  rpm --root /rootfs -qa --queryformat "%{NAME}-%{VERSION}-%{RELEASE}\n"
  rpm -q --queryformat "%{NAME}-%{VERSION}-%{RELEASE}\n" java-17-openjdk-portable
} | sort > /rootfs/stackable/package_manifest.txt

# dnf state and caches have no use in the final image (the rpmdb is kept)
rm -rf /rootfs/var/cache /rootfs/var/lib/dnf /rootfs/var/log/*

chown -R ${STACKABLE_USER_UID}:0 /rootfs/stackable
chown -h ${STACKABLE_USER_UID}:0 /rootfs/stackable/zookeeper
chmod -R g=u /rootfs/stackable

chroot /rootfs /usr/bin/check-permissions-ownership.sh /stackable ${STACKABLE_USER_UID} 0
EOF

# ==============================================================================
# FINAL ZOOKEEPER IMAGE

FROM scratch

ARG PRODUCT_VERSION
ARG RELEASE_VERSION
ARG STACKABLE_USER_UID

COPY --from=rootfs /rootfs /

LABEL \
  name="Apache ZooKeeper" \
  maintainer="info@stackable.tech" \
  vendor="Stackable GmbH" \
  version="${PRODUCT_VERSION}" \
  release="${RELEASE_VERSION}" \
  summary="The Stackable image for Apache ZooKeeper." \
  description="This image is deployed by the Stackable Operator for Apache ZooKeeper."

LABEL org.opencontainers.image.authors="info@stackable.tech"
LABEL org.opencontainers.image.url="https://stackable.tech"
LABEL org.opencontainers.image.source="https://github.com/stackabletech/docker-images/"
LABEL org.opencontainers.image.vendor="Stackable GmbH"
LABEL org.opencontainers.image.licenses="Apache-2.0"

# C.UTF-8 is built into glibc >= 2.35, so no langpack is needed (the Hummingbird images
# set LANG=en_US.UTF-8 but do not ship that locale)
ENV LANG=C.UTF-8
ENV JAVA_HOME=/usr/lib/jvm/java-17
ENV PATH="/usr/lib/jvm/java-17/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/stackable:/stackable/zookeeper/bin"
ENV ZOOKEEPER_HOME=/stackable/zookeeper
ENV HOME=/stackable
ENV LOG4J_FORMAT_MSG_NO_LOOKUPS=true

# Mitigation for CVE-2026-114938, copied from java-base/Dockerfile
ARG TARGETARCH
ARG IS_AMD64=${TARGETARCH%arm64}
ENV JAVA_TOOL_OPTIONS="${IS_AMD64:+-XX:-UseBASE64Intrinsics}"

USER ${STACKABLE_USER_UID}
WORKDIR /stackable/zookeeper
CMD ["bin/zkServer.sh", "start-foreground", "conf/zoo_sample.cfg"]
