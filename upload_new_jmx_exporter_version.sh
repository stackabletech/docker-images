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

src_file=jmx_prometheus-$VERSION-src.tar.gz

# JMX Exporter does not currently publish signatures or SBOMs (as of 2023-07-24, latest version at this point 0.19.0)
echo "Downloading JMX Exporter"
# JMX Exporter provides no offficial source tarballs, download from Git
git clone https://github.com/prometheus/jmx_exporter "jmx_prometheus-${VERSION}" "--branch=${VERSION}" --depth=1

echo "Archiving JMX Exporter"
git -C "jmx_prometheus-${VERSION}" archive "${VERSION}" --format=tar.gz --prefix="jmx_prometheus-${VERSION}-src/" > "${src_file}"
sha256sum "${src_file}" | cut --delimiter=' ' --field=1 > "${src_file}.sha256"

echo "Uploading to Nexus"
curl --fail -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "${src_file}" 'https://repo.stackable.tech/repository/packages/jmx-exporter/'
curl --fail -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "${src_file}.sha256" 'https://repo.stackable.tech/repository/packages/jmx-exporter/'

echo "Successfully uploaded new version of JMX Exporter ($VERSION) to Nexus"
echo "https://repo.stackable.tech/service/rest/repository/browse/packages/jmx-exporter/"
echo "https://github.com/prometheus/jmx_exporter/releases/tag/parent-$VERSION"
