#!/bin/bash

eval "$_STACKABLE_PRE_HOOK"

/stackable/spark/kubernetes/dockerfiles/spark/entrypoint.sh "$@"
result=$?

eval "$_STACKABLE_POST_HOOK"

exit $result
