#!/usr/bin/env bash

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

src_file=trino-storage-$VERSION-src.tar.gz

echo "Downloading Trino Storage"
# Trino Storage provides no offficial source tarballs, download from Git
git clone https://github.com/snowlift/trino-storage "trino-storage-${VERSION}" "--branch=v${VERSION}" --depth=1

echo "Archiving Trino Storage"
git -C "trino-storage-${VERSION}" archive "v${VERSION}" --format=tar.gz --prefix="trino-storage-${VERSION}-src/" > "${src_file}"
sha256sum "${src_file}" | cut --delimiter=' ' --field=1 > "${src_file}.sha256"

echo "Uploading everything to Nexus"
EXIT_STATUS=0
curl --fail -o /dev/null --progress-bar -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "${src_file}" 'https://repo.stackable.tech/repository/packages/trino-storage/' || EXIT_STATUS=$?
curl --fail -o /dev/null --progress-bar -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "${src_file}.sha256" 'https://repo.stackable.tech/repository/packages/trino-storage/' || EXIT_STATUS=$?

if [ $EXIT_STATUS -ne 0 ]; then
  echo "ERROR: Upload failed"
  exit 1
fi

echo "Successfully uploaded version ${VERSION} of Trino Storage to Nexus"
echo "https://repo.stackable.tech/service/rest/repository/browse/packages/trino-storage/"
