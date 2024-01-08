#!/bin/bash
set -euo pipefail
set -x

#PGPASSWORD=secret
export PGHOST=localhost
export PGUSER=postgres
export PGDATABASE=postgres
export DB_SCHEMA=feature_tracker

REGISTRY=docker.stackable.tech # REGISTRY=oci.stackable.tech
REGISTRY_PATH=stackable        # REGISTRY_PATH=sdp
PRODUCT_CODE_NAME=airflow
PRODUCT_VERSION=2.6.1
RELEASE_NAME=2023-11
RELEASE_VERSION=23.11.0

IMAGE_TAG=$PRODUCT_VERSION-stackable$RELEASE_VERSION

IMAGE_DIGEST=$(crane digest $REGISTRY/$REGISTRY_PATH/$PRODUCT_CODE_NAME:$IMAGE_TAG)

if [ -z "$IMAGE_DIGEST" ]; then
    echo "Image $REGISTRY/$REGISTRY_PATH/$PRODUCT_CODE_NAME:$IMAGE_TAG not found in registry"
    exit 1
fi

PURL="pkg:docker/$REGISTRY_PATH/$PRODUCT_CODE_NAME@$IMAGE_DIGEST?repository_url=$REGISTRY"

echo "PURL is $PURL"

# -AtR "": in this case this basically means "just output the content of the selected column"
# -c: execute the provided command and exit
RELEASE_ID=$(psql -AtR "" -c "SELECT id FROM $DB_SCHEMA.releases WHERE name = '$RELEASE_NAME'")

if [ -z "$RELEASE_ID" ]; then
    echo "Release $RELEASE_NAME not found in database"
    exit 1
fi

# create release version if it does not exist
psql -c "INSERT INTO $DB_SCHEMA.release_versions (release_id, version, created_at) VALUES ($RELEASE_ID, '$RELEASE_VERSION', now()) ON CONFLICT DO NOTHING"

PRODUCT_ID=$(psql -AtR "" -c "SELECT id FROM $DB_SCHEMA.products WHERE code_name = '$PRODUCT_CODE_NAME'")

if [ -z "$PRODUCT_ID" ]; then
    echo "Product $PRODUCT_CODE_NAME not found in database"
    exit 1
fi

PRODUCT_VERSION_ID=$(psql -AtR "" -c "SELECT id FROM $DB_SCHEMA.product_versions WHERE product_id = $PRODUCT_ID AND version = '$PRODUCT_VERSION'")

if [ -z "$PRODUCT_VERSION_ID" ]; then
    echo "Product version $PRODUCT_VERSION not found in database, creating it now"
    PRODUCT_VERSION_ID=$(psql -AtR "" -c "INSERT INTO $DB_SCHEMA.product_versions (product_id, version) VALUES ($PRODUCT_ID, '$PRODUCT_VERSION') RETURNING id" | head -n1)
fi

psql -c "INSERT INTO $DB_SCHEMA.release_components (product_version_id, release_version, purl) VALUES ($PRODUCT_VERSION_ID, '$RELEASE_VERSION', '$PURL')"
