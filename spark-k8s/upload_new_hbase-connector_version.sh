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

download_url="https://github.com/apache/hbase-connectors/archive/refs/tags/rel/${VERSION}.tar.gz"

tar_gz_file="hbase-connectors_${VERSION}.tar.gz"

echo "Downloading hbase-connectors source from ${download_url}"
curl --fail -L -o "${tar_gz_file}" "${download_url}"

echo "Uploading hbase-connectors source to Nexus"
EXIT_STATUS=0
curl --fail -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "${tar_gz_file}" 'https://repo.stackable.tech/repository/packages/hbase-connectors/' || EXIT_STATUS=$?

if [ $EXIT_STATUS -ne 0 ]; then
  echo "ERROR: Upload failed"
  exit 1
fi

echo "Successfully uploaded version $VERSION of hbase-connectors to Nexus"
echo "https://repo.stackable.tech/service/rest/repository/browse/packages/hbase-connectors/"
