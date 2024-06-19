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

bin_file="apache-hive-${VERSION}-bin.tar.gz"
src_file="apache-hive-$VERSION-src.tar.gz"

echo "Downloading Hive (this can take a while, it is intentionally downloading from a slow mirror that contains all old versions)"
curl --fail -LOs "https://dlcdn.apache.org/hive/hive-${VERSION}/${bin_file}"
curl --fail -LOs "https://dlcdn.apache.org/hive/hive-${VERSION}/${bin_file}.asc"
curl --fail -LOs "https://dlcdn.apache.org/hive/hive-${VERSION}/${bin_file}.sha256"

echo "Downloading Hive (this can take a while, it is intentionally downloading from a slow mirror that contains all old versions)"
curl --fail -LOs "https://dlcdn.apache.org/hive/hive-${VERSION}/${src_file}"
curl --fail -LOs "https://dlcdn.apache.org/hive/hive-${VERSION}/${src_file}.asc"
curl --fail -LOs "https://dlcdn.apache.org/hive/hive-${VERSION}/${src_file}.sha256"


# It is probably redundant to check both the checksum and the signature but it's cheap and why not
echo "Validating SHA256 Checksums"
if ! (sha256sum "${bin_file}" | diff - "${bin_file}.sha256" && sha256sum "${src_file}" | diff - "${src_file}.sha256"); then
  echo "ERROR: One of the SHA256 sum does not match"
  exit 1
fi

echo "Validating signatures"
echo '--> NOTE: Make sure you have downloaded and added the KEYS file (https://dlcdn.apache.org/hive/KEYS) to GPG: https://www.apache.org/info/verification.html (e.g. by using "curl https://dlcdn.apache.org/hive/KEYS | gpg --import")'

if ! (gpg --verify "$bin_file.asc" "$bin_file" 2> /dev/null &&  gpg --verify "$src_file.asc" "$src_file" 2> /dev/null); then
  echo "ERROR: Signature could not be verified"
  exit 1
fi

echo "Uploading everything to Nexus"
EXIT_STATUS=0
curl --fail -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "$bin_file" 'https://repo.stackable.tech/repository/packages/hive/' || EXIT_STATUS=$?
curl --fail -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "$bin_file.asc" 'https://repo.stackable.tech/repository/packages/hive/' || EXIT_STATUS=$?
curl --fail -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "$bin_file.sha256" 'https://repo.stackable.tech/repository/packages/hive/' || EXIT_STATUS=$?

curl --fail -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "$src_file" 'https://repo.stackable.tech/repository/packages/hive/' || EXIT_STATUS=$?
curl --fail -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "$src_file.asc" 'https://repo.stackable.tech/repository/packages/hive/' || EXIT_STATUS=$?
curl --fail -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "$src_file.sha256" 'https://repo.stackable.tech/repository/packages/hive/' || EXIT_STATUS=$?


if [ $EXIT_STATUS -ne 0 ]; then
  echo "ERROR: Upload failed"
  exit 1
fi

echo "Successfully uploaded version $VERSION of Hive to Nexus"
echo "https://repo.stackable.tech/service/rest/repository/browse/packages/hive/"
