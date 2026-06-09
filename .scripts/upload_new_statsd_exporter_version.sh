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

# statsd_exporter does not currently publish signatures or SBOMs
DOWNLOAD_URL="https://github.com/prometheus/statsd_exporter/archive/refs/tags/v$VERSION.tar.gz"

echo "Downloading statsd_exporter source"
if ! curl --fail -Ls -O "$DOWNLOAD_URL"; then
  echo "Failed to download from $DOWNLOAD_URL"
  exit 1
fi

FILE_NAME="statsd_exporter-$VERSION.src.tar.gz"
mv "v$VERSION.tar.gz" "$FILE_NAME"

echo "Uploading statsd_exporter source to Nexus"
if ! curl --fail -o /dev/null --progress-bar -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "$FILE_NAME" 'https://repo.stackable.tech/repository/packages/statsd_exporter/'; then
  echo "Failed to upload statsd_exporter source to Nexus"
  exit 1
fi

echo "Successfully uploaded new version of statsd_exporter source ($VERSION) to Nexus"
echo "https://repo.stackable.tech/service/rest/repository/browse/packages/statsd_exporter/"
