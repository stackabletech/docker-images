#!/usr/bin/env bash

AIRFLOW_VERSION=${1:?"Missing version number argument (arg 1)"}
PYTHON_VERSION=${2:-"3.9"}

URL="https://raw.githubusercontent.com/apache/airflow/constraints-${AIRFLOW_VERSION}/constraints-${PYTHON_VERSION}.txt"
FILENAME="constraints-${AIRFLOW_VERSION}-python${PYTHON_VERSION//.}.txt"

# Find the directory name of the script so it can be run from outside
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
pushd ${DIR} > /dev/null

echo "Downloading constraints file for Airflow ${AIRFLOW_VERSION} (Python ${PYTHON_VERSION})"
curl --fail -Ls "${URL}" -o "${FILENAME}"

echo "Successfully pulled new constraints file: ${FILENAME}"
popd > /dev/null
