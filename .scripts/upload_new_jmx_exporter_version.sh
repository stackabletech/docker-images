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

JAR_FILE="jmx_prometheus_javaagent-$VERSION.jar"
SUM_FILE="$JAR_FILE.sha256"

echo "Downloading JMX Exporter"
curl --fail -LO --progress-bar "https://github.com/prometheus/jmx_exporter/releases/download/$VERSION/$JAR_FILE"
curl --fail -LO --progress-bar "https://github.com/prometheus/jmx_exporter/releases/download/$VERSION/$SUM_FILE"

# Check that sha256 sum matches before uploading
sha256sum --check --status "$SUM_FILE" && echo "SHA256 Sum matches"

echo "Uploading to Nexus"
curl --fail -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "$JAR_FILE" 'https://repo.stackable.tech/repository/packages/jmx-exporter/'
curl --fail -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "$SUM_FILE" 'https://repo.stackable.tech/repository/packages/jmx-exporter/'

echo "Successfully uploaded new version of the JMX Exporter ($VERSION) Jar to Nexus"
echo "https://repo.stackable.tech/service/rest/repository/browse/packages/jmx-exporter/"
echo "https://github.com/prometheus/jmx_exporter/releases/tag/$VERSION"
