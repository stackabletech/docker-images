# syntax=docker/dockerfile:1.10.0@sha256:865e5dd094beca432e8c0a1d5e1c465db5f998dca4e439981029b3b81fb39ed5
# check=error=true

#
# Base image for builder stages
#

# Find the latest version at https://catalog.redhat.com/software/containers/ubi9/ubi-minimal/615bd9b4075b022acc111bf5?container-tabs=gti
# IMPORTANT: Make sure to use the "Manifest List Digest" that references the images for multiple architectures
# rather than just the "Image Digest" that references the image for the selected architecture.
FROM registry.access.redhat.com/ubi9/ubi-minimal@sha256:c0e70387664f30cd9cf2795b547e4a9a51002c44a4a86aa9335ab030134bf392

# intentionally unused
ARG PRODUCT
ARG STACKABLE_USER_UID
ARG STACKABLE_USER_GID
ARG STACKABLE_USER_NAME

# This SHOULD be kept in sync with operator-templating and other tools to reduce build times
# Find the latest version here: https://doc.rust-lang.org/stable/releases.html
# renovate: datasource=github-releases packageName=rust-lang/rust
ENV RUST_DEFAULT_TOOLCHAIN_VERSION=1.84.1
# Find the latest version here: https://crates.io/crates/cargo-cyclonedx
# renovate: datasource=crate packageName=cargo-cyclonedx
ENV CARGO_CYCLONEDX_CRATE_VERSION=0.5.7
# Find the latest version here: https://crates.io/crates/cargo-auditable
# renovate: datasource=crate packageName=cargo-auditable
ENV CARGO_AUDITABLE_CRATE_VERSION=0.6.6

RUN <<EOF
microdnf update --assumeyes

# git: Needed to fetch source
# gcc: Needed for compilation
microdnf --assumeyes install \
  gcc \
  git

###
### Add Stackable user and group
###
# Added only temporarily to create the user and group, removed again below
microdnf --assumeyes install shadow-utils
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
microdnf --assumeyes remove shadow-utils

microdnf clean all
rm -rf /var/cache/yum

# WARNING (@NickLarsenNZ): We should pin the rustup version
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain "$RUST_DEFAULT_TOOLCHAIN_VERSION"
. "$HOME/.cargo/env" && cargo --quiet install cargo-cyclonedx@"$CARGO_CYCLONEDX_CRATE_VERSION" cargo-auditable@"$CARGO_AUDITABLE_CRATE_VERSION" && rustup toolchain install
EOF


# **patchable**
# Tool for patch management
# Copy source code of patchable
COPY rust/patchable/ /patchable/rust/patchable
# Copy workspace files
COPY Cargo.* /patchable/

RUN <<EOF
microdnf update --assumeyes
microdnf install --assumeyes openssl-devel pkg-config
cd /patchable
. "$HOME/.cargo/env"
cargo auditable --quiet build --release && cargo cyclonedx --all --spec-version 1.5 --describe binaries
mv /patchable/target/release/patchable /stackable/patchable
microdnf clean all
EOF
