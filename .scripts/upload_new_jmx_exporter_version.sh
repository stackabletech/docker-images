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

# JMX Exporter does not currently publish signatures or SBOMs (as of 2023-07-24, latest version at this point 0.19.0)
echo "Downloading JMX Exporter"
curl --fail -LOs "https://repo1.maven.org/maven2/io/prometheus/jmx/jmx_prometheus_javaagent/$VERSION/jmx_prometheus_javaagent-$VERSION.jar"

echo "Uploading to Nexus"
curl --fail -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "jmx_prometheus_javaagent-$VERSION.jar" 'https://repo.stackable.tech/repository/packages/jmx-exporter/'

echo "Successfully uploaded new version of JMX Exporter ($VERSION) to Nexus"
echo "https://repo.stackable.tech/service/rest/repository/browse/packages/jmx-exporter/"
echo "https://github.com/prometheus/jmx_exporter/releases/tag/parent-$VERSION"
