# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

# The boost library is a runtime dependency of the Hadoop native code
# but is not available in the RedHat UBI10 or EPEL10 repositories, so we need to build
# it ourselves.
#
# NOTE: We use a published source bundle instead of the patchable workflow
#       because boost uses git submodules for it's build system
#       and patchable doesn't support these.
#       The source bundle contains everything needed to build b2 (the build system)
#       and boost except for dependencies which are automatically discovered.
#
FROM local-image/stackable-base AS boost-builder

ARG BOOST_VERSION

WORKDIR /tmp

RUN <<EOF
microdnf install \
      gcc gcc-c++ make perl python3 \
      tar bzip2 xz
microdnf clean all

BOOST_UNDERSCORE="$(echo "${BOOST_VERSION}" | tr '.' '_')"
BOOST_TARBALL="boost_${BOOST_UNDERSCORE}.tar.bz2"
# See .scripts/upload_new_boost_version.sh for how this artifact got there.
BOOST_URL="https://repo.stackable.tech/repository/packages/boost/${BOOST_TARBALL}"

mkdir -p /stackable/boost
curl -fsSL "${BOOST_URL}" -o "${BOOST_TARBALL}"
tar -xf "${BOOST_TARBALL}"
cd "boost_${BOOST_UNDERSCORE}"
./bootstrap.sh --prefix=/stackable/boost
./b2 -j"$(nproc)" \
  link=shared runtime-link=shared variant=release threading=multi \
  install
rm -rf /tmp/*
EOF
