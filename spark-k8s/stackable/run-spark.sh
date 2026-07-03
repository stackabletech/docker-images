#!/bin/bash

  eval "$_STACKABLE_PRE_HOOK"

  # Forward SIGTERM to the Spark entrypoint to support spark JVM's gracefully shutdown.
  /stackable/spark/kubernetes/dockerfiles/spark/entrypoint.sh "$@" &
  child_pid=$!

  _handle_term() {
    kill -TERM "$child_pid" 2>/dev/null || true
  }
  trap _handle_term TERM INT

  # `wait` returns immediately when the trap fires; loop until the child is actually gone.
  wait "$child_pid"
  while kill -0 "$child_pid" 2>/dev/null; do
    wait "$child_pid" 2>/dev/null || true
  done
  result=$?

  eval "$_STACKABLE_POST_HOOK"

  exit $result
