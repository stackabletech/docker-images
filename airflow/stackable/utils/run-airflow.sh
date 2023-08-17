#!/bin/bash

eval "$_STACKABLE_PRE_HOOK"

/entrypoint.sh "$@"
result=$?

eval "$_STACKABLE_POST_HOOK"

exit $result
