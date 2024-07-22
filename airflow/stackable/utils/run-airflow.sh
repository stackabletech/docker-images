#!/bin/bash
# This wrapper script allows commands that are embedded in environment variables
# to be invoked to carry out actions when the main container is complete.
# N.B. for airflow services (scheduler, webserver, worker) this will not be relevant
# as the service has to be terminated, but for pods spawned and terminated by the
# kubernetes executor this allows us to stop other containers, such as vector.
eval "$_STACKABLE_PRE_HOOK"

/entrypoint.sh "$@"
result=$?

eval "$_STACKABLE_POST_HOOK"

exit $result
