#!/usr/bin/env bash

VERSION=${1:?"Missing version number argument (arg 1)"}
NEXUS_USER=${2:?"Missing Nexus username argument (arg 2)"}

ARCHITECTURES=(
    arm64
    amd64
)

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

for arch in "${ARCHITECTURES[@]}"; do
  # Statsd_exporter does not currently publish signatures or SBOMs
  echo "Downloading STATSD EXPORTER"
  curl --fail -LOs "https://github.com/prometheus/statsd_exporter/releases/download/v$VERSION/statsd_exporter-$VERSION.linux-$arch.tar.gz"

  echo "Uploading to Nexus"
  curl --fail -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "statsd_exporter-$VERSION.linux-$arch.tar.gz" 'https://repo.stackable.tech/repository/packages/statsd_exporter/'
done

echo "Successfully uploaded new version of STATSD-EXPORTER ($VERSION) to Nexus"
echo "https://repo.stackable.tech/service/rest/repository/browse/packages/statsd_exporter/"
