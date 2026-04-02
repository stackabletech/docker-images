#!/usr/bin/env bash

set -euo pipefail

VERSION=${1:?"Missing version number argument (arg 1)"}
MAJOR=$(echo "$VERSION" | grep -oE '^[0-9]+')
NEXUS_USER=${2:?"Missing Nexus username argument (arg 2)"}

read -r -s -p "Nexus Password: " NEXUS_PASSWORD
echo

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

# Example download URLs found at https://maven.apache.org/download.cgi
# https://dlcdn.apache.org/maven/maven-3/3.9.11/binaries/apache-maven-3.9.11-bin.tar.gz
# https://downloads.apache.org/maven/maven-3/3.9.11/binaries/apache-maven-3.9.11-bin.tar.gz.sha512
# https://downloads.apache.org/maven/maven-3/3.9.11/binaries/apache-maven-3.9.11-bin.tar.gz.asc
# https://downloads.apache.org/maven/KEYS
# Preferring downloads.apache.org over dlcdn.apache.org (for no real reason, but wanted consistency)

BASE_URL="https://downloads.apache.org/maven/maven-$MAJOR/$VERSION/binaries"
ARCHIVE_FILE="apache-maven-$VERSION-bin.tar.gz"
SUM_FILE="$ARCHIVE_FILE.sha512"
SIG_FILE="$ARCHIVE_FILE.asc"

echo "Downloading Maven $VERSION"
curl --fail -LO --progress-bar "$BASE_URL/$ARCHIVE_FILE"
curl --fail -LO --progress-bar "$BASE_URL/$SUM_FILE"
curl --fail -LO --progress-bar "$BASE_URL/$SIG_FILE"

# Maven maintainers produce sum files that are incompatible with sha*sum, so we
# need to append the archive name to the end to make it work.
echo -n "  $ARCHIVE_FILE" >> "$SUM_FILE"

# Check that sha512 sum matches before uploading
sha512sum --strict --check --status "$SUM_FILE" # do not put && here
echo "SHA512 Sum matches"

if ! gpg --verify "$SIG_FILE" "$ARCHIVE_FILE"; then
  echo "You might need to download the public keys and try again:"
  echo "curl https://downloads.apache.org/maven/KEYS | gpg --import"
  exit 1
fi

echo "Uploading to Nexus"

curl --fail -o /dev/null --progress-bar -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "$ARCHIVE_FILE" 'https://repo.stackable.tech/repository/packages/maven/'
curl --fail -o /dev/null --progress-bar -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "$SUM_FILE" 'https://repo.stackable.tech/repository/packages/maven/'
curl --fail -o /dev/null --progress-bar -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "$SIG_FILE" 'https://repo.stackable.tech/repository/packages/maven/'

echo "Successfully uploaded Maven $VERSION to Nexus"
echo "https://repo.stackable.tech/service/rest/repository/browse/packages/maven/"
echo "https://github.com/prometheus/maven/releases/tag/$VERSION"
