# syntax=docker/dockerfile:1.16.0@sha256:e2dd261f92e4b763d789984f6eab84be66ab4f5f08052316d8eb8f173593acf7
# check=error=true

# The boost library is a runtime dependency of the Hadoop native code
# but is not available in the RedHat UBI10 or EPEL10 repositories, so we need to build
# it ourselves.
# Additionally, UBI ships a single supported boost version, which works for some product versions,
# but not for all.
#
# NOTE: We use a published source bundle instead of the patchable workflow
#       because boost uses git submodules for its build system
#       and patchable doesn't support these.
#       The source bundle contains everything needed to build b2 (the build system)
#       and boost except for dependencies which are automatically discovered.
#
FROM local-image/stackable-base AS boost-builder

ARG BOOST_VERSION

WORKDIR /tmp

RUN <<EOF
microdnf install gcc gcc-c++ make perl python3 tar bzip2 xz
microdnf clean all

BOOST_UNDERSCORE="$(echo "${BOOST_VERSION}" | tr '.' '_')"
BOOST_TARBALL="boost_${BOOST_UNDERSCORE}.tar.bz2"
# See .scripts/upload_new_boost_version.sh for how this artifact got there.
BOOST_URL="https://repo.stackable.tech/repository/packages/boost/${BOOST_TARBALL}"

mkdir -p /stackable/boost
curl -fsSL "${BOOST_URL}" -o "${BOOST_TARBALL}"
tar -xf "${BOOST_TARBALL}"
cd "boost_${BOOST_UNDERSCORE}"

# glibc >= 2.34 makes PTHREAD_STACK_MIN a sysconf() call instead of a compile-time
# constant, which breaks the `#if PTHREAD_STACK_MIN > 0` preprocessor check in older
# boost versions (e.g. 1.72.0). Apply the upstream fix: use `#ifdef`.
# See: https://github.com/boostorg/thread/commit/74fb0a26099bc51d717f5f154b37231ce7df3e98
#
# Can be removed once we drop boost 1.72.0 support.
THREAD_DATA="boost/thread/pthread/thread_data.hpp"
if [ -f "${THREAD_DATA}" ]; then
  sed -i \
    -e 's/^#if PTHREAD_STACK_MIN > 0$/#ifdef PTHREAD_STACK_MIN/' \
    "${THREAD_DATA}"
fi

./bootstrap.sh --prefix=/stackable/boost
./b2 -j"$(nproc)" \
  link=shared runtime-link=shared variant=release threading=multi \
  install
rm -rf /tmp/*
EOF
