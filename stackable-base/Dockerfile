# syntax=docker/dockerfile:1.8.1@sha256:e87caa74dcb7d46cd820352bfea12591f3dba3ddc4285e19c7dcd13359f7cefd

# Manifest list digest because of multi architecture builds ( https://www.redhat.com/architect/pull-container-image#:~:text=A%20manifest%20list%20exists%20to,system%20on%20a%20specific%20architecture )
FROM registry.access.redhat.com/ubi9-minimal@sha256:a7d837b00520a32502ada85ae339e33510cdfdbc8d2ddf460cc838e12ec5fa5a AS product-utils-builder

ENV CONFIG_UTILS_VERSION=0.2.0
# This SHOULD be kept in sync with operator-templating and other tools to reduce build times
ENV RUST_DEFAULT_TOOLCHAIN_VERSION=1.79.0
ENV CARGO_CYCLONEDX_CRATE_VERSION=0.4.0
ENV CARGO_AUDITABLE_CRATE_VERSION=0.6.4

RUN <<EOF
microdnf update --assumeyes

# git: Needed to fetch source
# gcc: Needed for compilation
microdnf --assumeyes install \
  gcc \
  git
microdnf clean all
rm -rf /var/cache/yum

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain $RUST_DEFAULT_TOOLCHAIN_VERSION
. "$HOME/.cargo/env" && cargo install cargo-cyclonedx@$CARGO_CYCLONEDX_CRATE_VERSION cargo-auditable@$CARGO_AUDITABLE_CRATE_VERSION

git clone --depth 1 --branch ${CONFIG_UTILS_VERSION} https://github.com/stackabletech/config-utils
cd ./config-utils
. $HOME/.cargo/env
cargo auditable build --release && cargo cyclonedx --output-pattern package --all --output-cdx
EOF

# Manifest list digest because of multi architecture builds ( https://www.redhat.com/architect/pull-container-image#:~:text=A%20manifest%20list%20exists%20to,system%20on%20a%20specific%20architecture )
FROM registry.access.redhat.com/ubi9-minimal@sha256:a7d837b00520a32502ada85ae339e33510cdfdbc8d2ddf460cc838e12ec5fa5a AS final

# intentionally unused
ARG PRODUCT

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

RUN <<EOF
microdnf update

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
  iputils \
  less \
  nano \
  tar

# Added only temporarily to create the user and group, removed again below
microdnf install shadow-utils
groupadd --gid 1000 --system stackable
# The --no-log-init is required to work around a bug/problem in Go/Docker when very large UIDs are used
# See https://github.com/moby/moby/issues/5419#issuecomment-41478290 for more context
useradd --no-log-init --gid stackable --uid 1000 --system stackable -d /stackable

mkdir /stackable
chown stackable:stackable /stackable
microdnf remove shadow-utils
microdnf clean all

echo "alias ll='ls -alF --color=auto'" >> /stackable/.bashrc
echo "alias ls='ls --color=auto'" >> /stackable/.bashrc
echo "alias ..='cd ..'" >> /stackable/.bashrc
echo "export PS1='\u@\[\e[36m\]\H\[\e[m\] \[\e[32m\]\$(pwd)\[\e[m\] \\$ '" >> /stackable/.bashrc
echo -e "if [ -f ~/.bashrc ]; then\n\tsource ~/.bashrc\nfi" >> /stackable/.profile
chown stackable:stackable /stackable/.bashrc
chown stackable:stackable /stackable/.profile

# CVE-2023-37920: Remove "e-Tugra" root certificates
# e-Tugra's root certificates were subject to an investigation prompted by reporting of security issues in their systems
# The package ca-certificates 2023.07.22 fixes the problem, until ubi9-minimal updates to it, we should remove them
# manually.
if [ "$(rpm -qa ca-certificates)" != "ca-certificates-2023.2.60_v7.0.306-90.1.el9_2.noarch" ]; then
  echo "The ca-certificates package was updated. Please check if the e-Tugra root certificates are present. \
When they have been removed, manually blacklisting them should be removed"
  echo "Let me help you by running trust list --filter=ca-anchors | grep 'E-Tugra'"
  trust list --filter=ca-anchors | grep 'E-Tugra'
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

COPY --from=product-utils-builder --chown=stackable:stackable /config-utils/target/release/config-utils /stackable/config-utils
COPY --from=product-utils-builder --chown=stackable:stackable /config-utils/config-utils.cdx.xml /stackable/config-utils.cdx.xml
ENV PATH="${PATH}:/stackable"

# These labels have mostly been superceded by the OpenContainer spec annotations below but it doesn't hurt to include them
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
