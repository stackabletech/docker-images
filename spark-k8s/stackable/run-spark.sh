#!/bin/bash

prepare_signal_handlers() {
  unset term_child_pid
  unset term_kill_needed
  trap handle_term_signal TERM INT
}

# Only ever invoked via the trap installed above.
# shellcheck disable=SC2329
handle_term_signal() {
  if [[ -v term_child_pid ]]; then
    kill -TERM "${term_child_pid}" 2>/dev/null || true
  else
    term_kill_needed="yes"
  fi
}

# Waits for the given child and returns *its* exit status.
# A trapped signal makes `wait` return immediately with a status > 128 while the child keeps
# running its shutdown sequence, so in that case wait again instead of taking the interrupted
# status.
wait_for_termination() {
  term_child_pid=$1
  if [[ -v term_kill_needed ]]; then
    kill -TERM "${term_child_pid}" 2>/dev/null || true
  fi
  while true; do
    wait "${term_child_pid}"
    term_child_status=$?
    if [[ "${term_child_status}" -gt 128 ]] && kill -0 "${term_child_pid}" 2>/dev/null; then
      continue
    fi
    return "${term_child_status}"
  done
}

eval "$_STACKABLE_PRE_HOOK"

# The signal handlers are installed before the child is started, so that a signal arriving in
# between is not lost. SIGTERM is forwarded to the Spark entrypoint to let the Spark JVM shut
# down gracefully.
prepare_signal_handlers

/stackable/spark/kubernetes/dockerfiles/spark/entrypoint.sh "$@" &

result=0
wait_for_termination $! || result=$?

eval "$_STACKABLE_POST_HOOK"

exit "${result}"
