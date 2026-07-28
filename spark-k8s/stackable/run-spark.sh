#!/bin/bash

eval "$_STACKABLE_PRE_HOOK"

# Forward SIGTERM to the Spark entrypoint to support spark JVM's gracefully shutdown.
/stackable/spark/kubernetes/dockerfiles/spark/entrypoint.sh "$@" &
child_pid=$!

# shellcheck disable=SC2329
_handle_term() {
  kill -TERM "$child_pid" 2>/dev/null || true
}
trap _handle_term TERM INT

# Wait for the entrypoint and propagate its exit status.
# A trapped signal makes `wait` return immediately with a status > 128 while the child keeps
# running, so in that case wait again rather than taking the interrupted status.
while true; do
  wait "$child_pid"
  result=$?
  if [ "$result" -gt 128 ] && kill -0 "$child_pid" 2>/dev/null; then
    continue
  fi
  break
done

eval "$_STACKABLE_POST_HOOK"

exit "$result"
