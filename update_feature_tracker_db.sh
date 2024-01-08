#!/bin/bash
# Update the release components table with image identifiers and
# product versions for a given release version.
#
# Requirements:
#
# 1. crane - Remote image handler
# 2. psql  - Postgres command line client.
#
# Environment:
#
# The following variables are read from the environment:
#
#  PGPASSWORD
#  PGHOST
#  PGUSER
#  PGDATABASE
#  DB_SCHEMA
#

set -euo pipefail
#set -x

PRODUCT_CODE_NAMES=(
	airflow
	druid
	hadoop
	hbase
	hive
	kafka
	nifi
	spark-k8s
	superset
	trino
	zookeeper
)

product_image_digest() {
	# Return the digest for the given image name.
	#
	# Example output: sha256:822d427031bf93055c51f4dc3bcaa468867ce83f59de6824af279df4cce2d066
	#
	# Params:
	#
	# IMAGE_NAME - Example: docker.stackable.tech/stackable/zookeeper:3.8.3-stackable23.11.0
	#
	IMAGE_NAME="$1"

	IMAGE_DIGEST=$(crane digest "${IMAGE_NAME}")

	if [ -z "$IMAGE_DIGEST" ]; then
		echo "Image ${IMAGE_NAME} not found in registry"
		exit 1
	fi

	echo "${IMAGE_DIGEST}"
}

ensure_release_version() {
	# Ensure the release version is present in the DB.
	#
	# Params:
	#
	# RELEASE_NAME    - Example: 2023-11
	# RELEASE_VERSION - Example: 23.11.0

	# -AtR "": in this case this basically means "just output the content of the selected column"
	# -c: execute the provided command and exit
	RELEASE_ID=$(psql -qAtR "" -c "SELECT id FROM $DB_SCHEMA.releases WHERE name = '$RELEASE_NAME'")

	if [ -z "$RELEASE_ID" ]; then
		echo "Release $RELEASE_NAME not found in database"
		exit 1
	fi

	# Create release version if it does not exist
	psql -q -c "INSERT INTO $DB_SCHEMA.release_versions (release_id, version, created_at) VALUES ($RELEASE_ID, '$RELEASE_VERSION', now()) ON CONFLICT DO NOTHING"
}

ensure_product_version() {
	# Ensure that a given product version exists in the DB.
	# Returns the product version id.
	#
	# Params:
	#
	# PRODUCT_CODE_NAME - See the PRODUCT_CODE_NAMES array for valid values.
	# PRODUCT_VERSION   - Example: 2.6.1
	#
	# Return:
	#
	# The id of the product version.
	#
	PRODUCT_CODE_NAME="$1"
	PRODUCT_VERSION="$2"

	PRODUCT_ID=$(psql -qAtR "" -c "SELECT id FROM $DB_SCHEMA.products WHERE code_name = '$PRODUCT_CODE_NAME'")

	if [ -z "$PRODUCT_ID" ]; then
		echo >&2 "Product $PRODUCT_CODE_NAME not found in database"
		exit 1
	fi

	PRODUCT_VERSION_ID=$(psql -qAtR "" -c "SELECT id FROM $DB_SCHEMA.product_versions WHERE product_id = $PRODUCT_ID AND version = '$PRODUCT_VERSION'")

	if [ -z "$PRODUCT_VERSION_ID" ]; then
		echo >&2 "Product version $PRODUCT_VERSION not found in database, creating it now"
		psql -qAtR "" -c "INSERT INTO $DB_SCHEMA.product_versions (product_id, version) VALUES ($PRODUCT_ID, '$PRODUCT_VERSION')"
	fi

	PRODUCT_VERSION_ID=$(psql -qAtR "" -c "SELECT id FROM $DB_SCHEMA.product_versions WHERE product_id = $PRODUCT_ID AND version = '$PRODUCT_VERSION'")

	echo "${PRODUCT_VERSION_ID}"
}

update_release_components() {
	# Update the release components table for the given parameters.
	#
	# Params:
	#
	# RELEASE_VERSION     - Example: 23.11.0
	# PRODUCT_VERSION_ID  - Example: 38. See ensure_product_version() above.
	# REPOSITORY_NAME     - Example: docker.stackable.tech/stackable/zookeeper
	# IMAGE_DIGEST        - Example: sha256:822d427031bf93055c51f4dc3bcaa468867ce83f59de6824af279df4cce2d066

	local RELEASE_VERSION="$1"
	local PRODUCT_VERSION_ID="$2"
	local REPOSITORY_NAME="$3"
	local IMAGE_DIGEST="$4"

	REGISTRY=$(echo "${REPOSITORY_NAME}" | sed 's/\/.*//')

	PURL="pkg:docker/${REPOSITORY_NAME}@${IMAGE_DIGEST}?repository_url=$REGISTRY"

	psql -q -c "INSERT INTO $DB_SCHEMA.release_components (product_version_id, release_version, purl) VALUES ($PRODUCT_VERSION_ID, '$RELEASE_VERSION', '$PURL') ON CONFLICT DO NOTHING"
}

usage() {
	cat <<EOF

  Usage: $0 <release> <version>

  Options:

  release - release name (ex. 2023-11)"
  version - release version (ex. 23.11.0)

  The following environment variables are required for the connection to the Postgres DB:

    PGPASSWORD
    PGHOST
    PGUSER
    PGDATABASE
    DB_SCHEMA

EOF
}

main() {
	# Update the release components for the given release (version).
	#
	# Params
	#
	# RELEASE_NAME    - Example: 2023-11
	# RELEASE_VERSION - Example: 23.11.0
	#
	local RELEASE_NAME="${1:-}"
	local RELEASE_VERSION="${2:-}"

	local REGISTRY=docker.stackable.tech # REGISTRY=oci.stackable.tech
	local REGISTRY_PATH=stackable        # REGISTRY_PATH=sdp

	if [ -z "$RELEASE_NAME" ]; then
		echo "Release name cannot be empty."
		usage
		exit 1
	fi

	if [ -z "$RELEASE_VERSION" ]; then
		echo "Release version cannot be empty."
		usage
		exit 1
	fi

	ensure_release_version "${RELEASE_NAME}" "${RELEASE_VERSION}"

	for PRODUCT_CODE_NAME in "${PRODUCT_CODE_NAMES[@]}"; do
		REPOSITORY_NAME="${REGISTRY}/${REGISTRY_PATH}/${PRODUCT_CODE_NAME}"

		IMAGES=$(crane ls "${REPOSITORY_NAME}" | grep "stackable${RELEASE_VERSION}" | xargs -I '{}' echo "${REPOSITORY_NAME}":'{}')

		for IMAGE_NAME in $(echo "${IMAGES}"); do
			PRODUCT_VERSION=$(echo "${IMAGE_NAME}" | sed 's/^.*://' | sed 's/-.*//')

			PRODUCT_VERSION_ID=$(ensure_product_version "${PRODUCT_CODE_NAME}" "${PRODUCT_VERSION}")

			PRODUCT_IMAGE_DIGEST=$(product_image_digest "${IMAGE_NAME}")

			update_release_components "${RELEASE_VERSION}" "${PRODUCT_VERSION_ID}" "${REPOSITORY_NAME}" "${PRODUCT_IMAGE_DIGEST}"
		done
	done
}

main "$@"
