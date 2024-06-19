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

bin_file=apache-zookeeper-$VERSION-bin.tar.gz
src_file=apache-zookeeper-$VERSION.tar.gz
download_url=https://archive.apache.org/dist/zookeeper

echo "Downloading ZooKeeper (this can take a while, it is intentionally downloading from a slow mirror that contains all old versions)"
curl --fail -LOs "$download_url/zookeeper-$VERSION/$bin_file"
curl --fail -LOs "$download_url/zookeeper-$VERSION/$bin_file.asc"
curl --fail -LOs "$download_url/zookeeper-$VERSION/$bin_file.sha512"

echo "Downloading ZooKeeper sources (this can take a while, it is intentionally downloading from a slow mirror that contains all old versions)"
curl --fail -LOs "$download_url/zookeeper-$VERSION/$src_file"
curl --fail -LOs "$download_url/zookeeper-$VERSION/$src_file.asc"
curl --fail -LOs "$download_url/zookeeper-$VERSION/$src_file.sha512"


# It is probably redundant to check both the checksum and the signature but it's cheap and why not
echo "Validating SHA512 Checksums for binary releases"
if ! (sha512sum "$bin_file" | diff -Z - "$bin_file.sha512"); then
  echo "ERROR: One of the SHA512 sums for the binary release does not match"
  exit 1
fi
echo "Validating SHA512 Checksums for source releases"
if ! (sha512sum "$src_file" | diff -Z - "$src_file.sha512"); then
  echo "ERROR: One of the SHA512 sums for the source release does not match"
  exit 1
fi

echo "Validating signatures for binary releases"
echo '--> NOTE: Make sure you have downloaded and added the KEYS file (https://archive.apache.org/dist/zookeeper/KEYS) to GPG: https://www.apache.org/info/verification.html (e.g. by using "curl https://archive.apache.org/dist/zookeeper/KEYS | gpg --import")'

if ! (gpg --verify "$bin_file.asc" "$bin_file" 2> /dev/null); then
  echo "ERROR: One of the signatures could not be verified for a binary release"
  exit 1
fi

echo "Validating signatures for source releases"
if ! (gpg --verify "$src_file.asc" "$src_file" 2> /dev/null); then
   echo "ERROR: One of the signatures could not be verified for a source release"
   exit 1
fi

echo "Uploading everything to Nexus"
EXIT_STATUS=0
repo_url=https://repo.stackable.tech/repository/packages/zookeeper/

curl --fail -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "$bin_file" "$repo_url" || EXIT_STATUS=$?
curl --fail -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "$bin_file.asc" "$repo_url" || EXIT_STATUS=$?
curl --fail -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "$bin_file.sha512" "$repo_url" || EXIT_STATUS=$?

curl --fail -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "$src_file" "$repo_url" || EXIT_STATUS=$?
curl --fail -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "$src_file.asc" "$repo_url" || EXIT_STATUS=$?
curl --fail -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "$src_file.sha512" "$repo_url" || EXIT_STATUS=$?

if [ $EXIT_STATUS -ne 0 ]; then
  echo "ERROR: Upload failed"
  exit 1
fi

echo "Successfully uploaded version $VERSION of ZooKeeper to Nexus"
echo "https://repo.stackable.tech/service/rest/repository/browse/packages/zookeeper/"
