#!/bin/bash

SPARK_CONTAINER_EXIT_CODE=0

eval "$_STACKABLE_PRE_HOOK"

cleanup() {
    eval "$_STACKABLE_POST_HOOK"
    exit $SPARK_CONTAINER_EXIT_CODE
}

trap 'cleanup' SIGTERM SIGINT

/stackable/spark/kubernetes/dockerfiles/spark/entrypoint.sh "$@"
SPARK_CONTAINER_EXIT_CODE=$?

cleanup
