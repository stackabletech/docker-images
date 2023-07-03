#!/usr/bin/env bash

VERSION=${1:?"Missing version number argument (arg 1)"}
NEXUS_USER=${2:?"Missing Nexus username argument (arg 2)"}

read -r -s -p "Nexus Password: " NEXUS_PASSWORD
echo ""

# Vector does not currently publish signatures or SBOMs (as of 2023-06-25, latest version at this point 0.30.0)

echo "Downloading X86 Vector"
curl -LOs "https://packages.timber.io/vector/$VERSION/vector-$VERSION-x86_64-unknown-linux-gnu.tar.gz"

echo "Downloading ARM Vector"
curl -LOs "https://packages.timber.io/vector/$VERSION/vector-$VERSION-aarch64-unknown-linux-gnu.tar.gz"

echo "Uploading both to Nexus"
curl --fail -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "vector-$VERSION-x86_64-unknown-linux-gnu.tar.gz" 'https://repo.stackable.tech/repository/packages/vector/'
curl --fail -u "$NEXUS_USER:$NEXUS_PASSWORD" --upload-file "vector-$VERSION-aarch64-unknown-linux-gnu.tar.gz" 'https://repo.stackable.tech/repository/packages/vector/'

rm "vector-$VERSION-x86_64-unknown-linux-gnu.tar.gz"
rm "vector-$VERSION-aarch64-unknown-linux-gnu.tar.gz"

echo "Successfully uploaded new version of Vector ($VERSION) to Nexus"
echo "https://repo.stackable.tech/service/rest/repository/browse/packages/vector/"
