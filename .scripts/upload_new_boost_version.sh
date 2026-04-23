#!/usr/bin/env bash
# This script mirrors the boostorg/boost source bundle for the given version to Nexus.
# The boost source bundle is architecture independent.
# It contains its own build system (b2) which is also built from source before building boost itself, so we don't need to worry about architecture specific builds.
# This artifact is used by the hadoop/boost local image.


set -euo pipefail

VERSION=${1:?"Missing version number argument (arg 1)"}
NEXUS_USER=${2:?"Missing Nexus username argument (arg 2)"}

read -r -s -p "Nexus Password: " NEXUS_PASSWORD
echo ""

# https://stackoverflow.com/questions/4632028/how-to-create-a-temporary-directory
# Find the directory name of the script
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# the temp directory used, within $DIR
WORK_DIR=$(mktemp -d -p "$DIR")

# check if tmp dir was created
if [[ ! "$WORK_DIR" || ! -d "$WORK_DIR" ]]; then
  echo "Could not create temp dir"
  exit 1
fi

# deletes the temp directory
function cleanup {
  rm -rf "$WORK_DIR"
}

# register the cleanup function to be called on the EXIT signal
trap cleanup EXIT

cd "$WORK_DIR" || exit

# boost does not currently publish signatures or SBOMs
BOOST_UNDERSCORE="$(echo "${VERSION}" | tr '.' '_')"
BOOST_TARBALL="boost_${BOOST_UNDERSCORE}.tar.bz2"
DOWNLOAD_URL="https://archives.boost.io/release/$VERSION/source/$BOOST_TARBALL"

echo "Downloading boost"
if ! curl --fail -Ls -O "$DOWNLOAD_URL"; then
  echo "Failed to download from $DOWNLOAD_URL"
  exit 1
fi

FILE_NAME=$(basename "$DOWNLOAD_URL")

echo "Uploading boost to Nexus"
if ! curl --fail -o /dev/null --progress-bar -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "$FILE_NAME" 'https://repo.stackable.tech/repository/packages/boost/'; then
  echo "Failed to upload boost to Nexus"
  exit 1
fi

echo "Successfully uploaded new version of boost ($VERSION) to Nexus"
echo "https://repo.stackable.tech/service/rest/repository/browse/packages/boost/"
