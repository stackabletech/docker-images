#!/usr/bin/env bash

set -euo pipefail

VERSION=${1:?"Missing version number argument (arg 1)"}
NEXUS_USER=${2:?"Missing Nexus username argument (arg 2)"}

# We prefer fast downloads...
BASE_DOWNLOAD_URL="https://dlcdn.apache.org/kafka"
# However, if the version is not available, use the slow archive instead:
# BASE_DOWNLOAD_URL="https://archive.apache.org/dist/kafka"

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

bin_file=kafka_2.13-$VERSION.tgz
src_file=kafka-$VERSION-src.tgz

echo "Downloading Kafka binary (if this fails, try switching the BASE_DOWNLOAD_URL to the archive)"
curl --fail -LO --progress-bar "${BASE_DOWNLOAD_URL}/$VERSION/$bin_file"
curl --fail -LO --progress-bar "${BASE_DOWNLOAD_URL}/$VERSION/$bin_file.asc"
curl --fail -LO --progress-bar "${BASE_DOWNLOAD_URL}/$VERSION/$bin_file.sha512"

echo "Downloading Kafka source (if this fails, try switching the BASE_DOWNLOAD_URL to the archive)"
curl --fail -LO --progress-bar "${BASE_DOWNLOAD_URL}/$VERSION/$src_file"
curl --fail -LO --progress-bar "${BASE_DOWNLOAD_URL}/$VERSION/$src_file.asc"
curl --fail -LO --progress-bar "${BASE_DOWNLOAD_URL}/$VERSION/$src_file.sha512"

# It is probably redundant to check both the checksum and the signature but it's cheap and why not
echo "Validating SHA512 Checksum"
if ! (gpg --print-md SHA512 "$bin_file" | diff - "$bin_file.sha512" && gpg --print-md SHA512 "$src_file" | diff - "$src_file.sha512" ); then
  echo "ERROR: The SHA512 sum does not match"
  exit 1
fi

echo "Validating signatures"
if ! (gpg --verify "$bin_file.asc" "$bin_file" 2> /dev/null && gpg --verify "$src_file.asc" "$src_file" 2> /dev/null); then
  echo "ERROR: One of the signatures could not be verified"
  echo "--> Make sure you have imported the KEYS file (${BASE_DOWNLOAD_URL}/KEYS) into GPG: https://www.apache.org/info/verification.html"
  echo "--> e.g. \"curl ${BASE_DOWNLOAD_URL}/KEYS | gpg --import\""
  exit 1
fi

echo "Uploading everything to Nexus"
EXIT_STATUS=0
curl --fail -o /dev/null --progress-bar -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "$bin_file" 'https://repo.stackable.tech/repository/packages/kafka/' || EXIT_STATUS=$?
curl --fail -o /dev/null --progress-bar -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "$bin_file.asc" 'https://repo.stackable.tech/repository/packages/kafka/' || EXIT_STATUS=$?
curl --fail -o /dev/null --progress-bar -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "$bin_file.sha512" 'https://repo.stackable.tech/repository/packages/kafka/' || EXIT_STATUS=$?

curl --fail -o /dev/null --progress-bar -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "$src_file" 'https://repo.stackable.tech/repository/packages/kafka/' || EXIT_STATUS=$?
curl --fail -o /dev/null --progress-bar -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "$src_file.asc" 'https://repo.stackable.tech/repository/packages/kafka/' || EXIT_STATUS=$?
curl --fail -o /dev/null --progress-bar -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "$src_file.sha512" 'https://repo.stackable.tech/repository/packages/kafka/' || EXIT_STATUS=$?

if [ $EXIT_STATUS -ne 0 ]; then
  echo "ERROR: Upload failed"
  exit 1
fi

echo "Successfully uploaded version $VERSION of Kafka to Nexus"
echo "https://repo.stackable.tech/service/rest/repository/browse/packages/kafka/"
