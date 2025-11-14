# syntax=docker/dockerfile:1.20.0@sha256:26147acbda4f14c5add9946e2fd2ed543fc402884fd75146bd342a7f6271dc1d
# check=error=true

#
# Base image for final stages
# Should not contain any components that are not needed (or helpful) at runtime
# Build time dependencies that are used in multiple images can be added to `stackable-devel` (or a derivative like `java-devel`) instead
#

FROM local-image/stackable-devel AS rust-binaries

# Find the latest version here: https://github.com/stackabletech/config-utils/tags
# renovate: datasource=github-tags packageName=stackabletech/config-utils
ENV CONFIG_UTILS_VERSION=0.2.0
# Find the latest version here: https://github.com/stackabletech/containerdebug/tags
# renovate: datasource=github-tags packageName=stackabletech/containerdebug
ENV CONTAINERDEBUG_VERSION=0.2.0
# Find the latest version here: https://github.com/stackabletech/secret-operator/tags
# I could not find support for prefixes or regex in https://docs.renovatebot.com/modules/datasource/github-tags/,
# so I was unable to add a renovate hint.
ENV CERT_TOOLS_VERSION=0.1.0


RUN <<EOF
. "$HOME/.cargo/env"

cd /
git clone --depth 1 --branch "${CONFIG_UTILS_VERSION}" https://github.com/stackabletech/config-utils
git clone --depth 1 --branch "${CONTAINERDEBUG_VERSION}" https://github.com/stackabletech/containerdebug
git clone --depth 1 --branch "cert-tools-${CERT_TOOLS_VERSION}" https://github.com/stackabletech/secret-operator
cd /config-utils
cargo auditable --quiet build --release && cargo cyclonedx --all --spec-version 1.5 --describe binaries
cd /containerdebug
cargo auditable --quiet build --release && cargo cyclonedx --all --spec-version 1.5 --describe binaries
cd /secret-operator
cargo auditable --quiet build --release --package cert-tools && cargo cyclonedx --all --spec-version 1.5 --describe binaries
EOF

# Find the latest version at https://catalog.redhat.com/software/containers/ubi9/ubi-minimal/615bd9b4075b022acc111bf5?container-tabs=gti
# IMPORTANT: Make sure to use the "Manifest List Digest" that references the images for multiple architectures
# rather than just the "Image Digest" that references the image for the selected architecture.
FROM registry.access.redhat.com/ubi9/ubi-minimal@sha256:2f06ae0e6d3d9c4f610d32c480338eef474867f435d8d28625f2985e8acde6e8 AS final

# intentionally unused
ARG PRODUCT_VERSION
ARG STACKABLE_USER_UID
ARG STACKABLE_USER_GID
ARG STACKABLE_USER_NAME

# Sets the default shell to Bash with strict error handling and robust pipeline processing.
# "-e": Exits immediately if a command exits with a non-zero status
# "-u": Treats unset variables as an error, preventing unexpected behavior from undefined variables.
# "-o pipefail": Causes a pipeline to return the exit status of the last command in the pipe that failed, ensuring errors in any part of a pipeline are not ignored.
# "-c": Allows the execution of commands passed as a string
# This is automatically inherited in all other Dockerfiles that use this unless it is overwritten
SHELL ["/bin/bash", "-euo", "pipefail", "-c"]

# We configure microdnf to not install weak dependencies in this file
# Not doing this caused the content of images to become unpredictable because
# based on which packages get updated by `microdnf update` new weak dependencies
# might be installed that were not present earlier (the ubi base image doesn't
# seem to install weak dependencies)
# This also affects the packages that are installed in our Dockerfiles (java as prime
# example).
# https://github.com/stackabletech/docker-images/pull/533
COPY stackable-base/stackable/dnf.conf /etc/dnf/dnf.conf

# Default curl configuration to avoid forgetting settings and to declutter the Dockerfiles
COPY stackable-base/stackable/curlrc /root/.curlrc

# echo won't expand escape sequences. Consider printf.
# https://github.com/koalaman/shellcheck/wiki/SC2028
# Reason: This is complaining about the complicated PS1 statement.
# It seems to work as intended so I'm not going to touch it!
# hadolint ignore=SC2028
RUN <<EOF
microdnf update

# **findutils**
# Needed to find all patch files, used in `apply_patches.sh`, and helpful for debugging
# Added 2024-10: Last vulnerability in 2007, only two vulnerabilities in total, a risk we accept
# https://nvd.nist.gov/vuln/search/results?form_type=Advanced&results_type=overview&search_type=all&isCpeNameSearch=false&cpe_vendor=cpe%3A%2F%3Agnu&cpe_product=cpe%3A%2F%3Agnu%3Afindutils
# cpe:2.3:a:gnu:findutils:*:*:*:*:*:*:*:*
#
# **iputils**
# To make debugging easier, includes things like ping
# Added 2024-03: We cannot find any vulnerabilities in the past years
# https://github.com/iputils/iputils
#
# **less**
# To make debugging easier
# Added 2024-03: less has seen three vulnerabilities between 2004 and 2022 which is a risk we're willing to accept for the added convenience
# https://nvd.nist.gov/vuln/search/results?form_type=Advanced&results_type=overview&search_type=all&isCpeNameSearch=false&cpe_vendor=cpe%3A%2F%3Agnu&cpe_product=cpe%3A%2F%3A%3Aless
# cpe:2.3:a:gnu:less:*:*:*:*:*:*:*:*
#
# **nano**
# To make debugging and changing things easier
# Added 2024-03: We checked and it has not seen any vulnerabilities since 2010 (as of 2024-03) we decided to accept it into our base image
# https://nvd.nist.gov/vuln/search/results?form_type=Advanced&results_type=overview&query=cpe%3A2.3%3Aa%3Agnu%3Anano&search_type=all&isCpeNameSearch=false
# cpe:2.3:a:gnu:nano:*:*:*:*:*:*:*:*
#
# **tar**
# To enable kubectl cp
# Added 2024-03: We checked and it has seen eight vulnerabilities since 2001, mostly minor and it's not in executable path so we decided to accept the risk
# https://nvd.nist.gov/vuln/search/results?adv_search=true&isCpeNameSearch=true&query=cpe%3A2.3%3Aa%3Agnu%3Atar%3A-%3A*%3A*%3A*%3A*%3A*%3A*%3A*
# cpe:2.3:a:gnu:tar:-:*:*:*:*:*:*:*
microdnf install \
  findutils \
  iputils \
  less \
  nano \
  tar

###
### Add Stackable user and group
###
# Added only temporarily to create the user and group, removed again below
microdnf install shadow-utils
groupadd --gid ${STACKABLE_USER_GID} --system ${STACKABLE_USER_NAME}

# The --no-log-init is required to work around a bug/problem in Go/Docker when very large UIDs are used
# See https://github.com/moby/moby/issues/5419#issuecomment-41478290 for more context
# Making this a system user prevents a mail dir from being created, expiry of passwords etc. but it will warn:
#   useradd warning: stackable's uid 1000 is greater than SYS_UID_MAX 999
# We can safely ignore this warning, to get rid of the warning we could change /etc/login.defs but that does not seem worth it
# We'll leave the home directory hardcoded to /stackable because I don't want to deal with which chars might be valid and which might not in user name vs. directory
useradd \
  --no-log-init \
  --gid ${STACKABLE_USER_GID} \
  --uid ${STACKABLE_USER_UID} \
  --system \
  --create-home \
  --home-dir /stackable \
   ${STACKABLE_USER_NAME}
microdnf remove shadow-utils
microdnf clean all

###
### Make shell usage in our containers "nicer"
###
{
  echo "alias ll='ls -alF --color=auto'"
  echo "alias ls='ls --color=auto'"
  echo "alias ..='cd ..'"
  echo "export PS1='\u@\[\e[36m\]\H\[\e[m\] \[\e[32m\]\$(pwd)\[\e[m\] \\$ '"
} >> /stackable/.bashrc

echo -e "if [ -f ~/.bashrc ]; then\n\tsource ~/.bashrc\nfi" >> /stackable/.profile

chown ${STACKABLE_USER_UID}:0 /stackable/.bashrc
chown ${STACKABLE_USER_UID}:0 /stackable/.profile

cp /root/.curlrc /stackable/.curlrc
chown ${STACKABLE_USER_UID}:0 /stackable/.curlrc

# CVE-2023-37920: Remove "e-Tugra" root certificates
# e-Tugra's root certificates were subject to an investigation prompted by reporting of security issues in their systems
# Until they are removed by default from ca-certificates, we should remove them manually
EXPECTED_CERTS_PACKAGE="ca-certificates-2024.2.69_v8.0.303-91.4.el9_4.noarch"
ACTUAL_CERTS_PACKAGE="$(rpm -qa ca-certificates)"
if [ "$ACTUAL_CERTS_PACKAGE" != "$EXPECTED_CERTS_PACKAGE" ]; then
  echo "The ca-certificates package was updated to $ACTUAL_CERTS_PACKAGE. Please check if the e-Tugra root certificates are present. \
When they have been removed, manually blacklisting them should be removed"
  echo "Let me help you by running trust list --filter=ca-anchors | grep 'E-Tugra'"
  trust list --filter=ca-anchors | grep 'E-Tugra'
  echo "If the cert appears above, please update the expected package: EXPECTED_CERTS_PACKAGE=\"$ACTUAL_CERTS_PACKAGE\""
  exit 1;
fi
EOF

COPY stackable-base/stackable/ca-cert-blocklist/ /etc/pki/ca-trust/source/blocklist/
RUN <<EOF
update-ca-trust
if [ "$(trust list --filter=ca-anchors | grep -c 'E-Tugra')" != "0" ]; then
  echo "Still found E-Tugra root certificates, this should not happen!"
  exit 1;
fi
EOF

# **config-utils**
# Utility to replace values in a file with values from environment variables and files
COPY --from=rust-binaries --chown=${STACKABLE_USER_UID}:0 /config-utils/target/release/config-utils /stackable/config-utils
COPY --from=rust-binaries --chown=${STACKABLE_USER_UID}:0 /config-utils/config-utils_bin.cdx.xml /stackable/config-utils.cdx.xml

# **containerdebug**
# Debug tool that logs generic system information.
COPY --from=rust-binaries --chown=${STACKABLE_USER_UID}:0 /containerdebug/target/release/containerdebug /stackable/containerdebug
COPY --from=rust-binaries --chown=${STACKABLE_USER_UID}:0 /containerdebug/containerdebug_bin.cdx.xml /stackable/containerdebug_bin.cdx.xml

# **cert-tools**
# A CLI tool to merge two truststores in PEM or PKCS12 format in such as way that they are accepted by the JVM
COPY --from=rust-binaries --chown=${STACKABLE_USER_UID}:0 /secret-operator/target/release/cert-tools /stackable/cert-tools
COPY --from=rust-binaries --chown=${STACKABLE_USER_UID}:0 /secret-operator/rust/cert-tools/cert-tools_bin.cdx.xml /stackable/cert-tools_bin.cdx.xml

# **check-permissions-ownership.sh**
# Bash script to check proper permissions and ownership requirements in the final Stackable images
COPY --chown=${STACKABLE_USER_UID}:0 shared/checks/check-permissions-ownership.sh /bin/check-permissions-ownership.sh

ENV PATH="${PATH}:/stackable"

# These labels have mostly been superceded by the OpenContainer spec annotations below but it doesn't hurt to include them
# http://label-schema.org/rc1/
LABEL maintainer="info@stackable.tech"
LABEL vendor="Stackable GmbH"

# Overwriting/Pinning UBI labels
# https://github.com/projectatomic/ContainerApplicationGenericLabels
LABEL vcs-ref=""
LABEL distribution-scope="public"
LABEL url="https://stackable.tech"
ARG TARGETARCH
LABEL architecture="${TARGETARCH}"
LABEL com.redhat.component=""
# It complains about it being an invalid label but RedHat uses it and we want to override it and it works....
# hadolint ignore=DL3048
LABEL com.redhat.license_terms=""
LABEL io.buildah.version=""
LABEL io.openshift.expose-services=""

# https://github.com/opencontainers/image-spec/blob/036563a4a268d7c08b51a08f05a02a0fe74c7268/annotations.md#annotations
LABEL org.opencontainers.image.authors="info@stackable.tech"
LABEL org.opencontainers.image.url="https://stackable.tech"
LABEL org.opencontainers.image.source="https://github.com/stackabletech/docker-images/"
LABEL org.opencontainers.image.vendor="Stackable GmbH"
LABEL org.opencontainers.image.licenses="Apache-2.0"
