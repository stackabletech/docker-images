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

echo "Downloading STATSD EXPORTER source code"
curl --fail -LOs "https://github.com/prometheus/statsd_exporter/archive/refs/tags/v$VERSION.tar.gz" && mv "v$VERSION.tar.gz" "statsd_exporter-$VERSION.src.tar.gz"

echo "Uploading to Nexus"
curl --fail -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "statsd_exporter-$VERSION.src.tar.gz" 'https://repo.stackable.tech/repository/packages/statsd_exporter/'

echo "Successfully uploaded new version of STATSD-EXPORTER source code ($VERSION) to Nexus"
echo "https://repo.stackable.tech/service/rest/repository/browse/packages/statsd_exporter/"
