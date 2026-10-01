#!/usr/bin/env bash
# Watch the build runs for a release tag and re-run failed jobs until they all
# pass. A single tag triggers one build run per product, and transient
# registry/network blips fail runs that a plain re-run would fix; this saves
# clicking "Re-run failed jobs" by hand.
#
# Every failure is retried up to --max-attempts and then given up on, so a real
# build break gets retried too. Check the log of anything that gives up.
#
# Needs: gh (authenticated) and jq.

set -euo pipefail

interval=60
max_attempts=10
dry_run=false
tag=

usage() {
  cat <<'EOF'
Usage: watch-builds.sh TAG [options]

  TAG                git tag to watch (required), e.g. 26.7.1

  --interval SECS    seconds between polls (default: 60)
  --max-attempts N   attempts per run before giving up (default: 10)
  --dry-run          list what would be re-run without triggering it
  -h, --help         show this help
EOF
}

while [ $# -gt 0 ]; do
  case $1 in
    --interval)     interval=$2; shift 2 ;;
    --max-attempts) max_attempts=$2; shift 2 ;;
    --dry-run)      dry_run=true; shift ;;
    -h|--help)      usage; exit 0 ;;
    -*)             echo "watch-builds.sh: unknown option: $1" >&2; usage >&2; exit 2 ;;
    *)              tag=$1; shift ;;
  esac
done

if [ -z "$tag" ]; then
  echo "watch-builds.sh: a TAG is required" >&2
  usage >&2
  exit 2
fi

for bin in gh jq; do
  command -v "$bin" >/dev/null || { echo "watch-builds.sh: $bin is not on PATH" >&2; exit 1; }
done

# Attempts already triggered per run id, seeded from each run's real run_attempt
# so re-runs done by hand in the UI still count towards the cap.
declare -A attempts
declare -A gave_up

# A run is worth re-running only if it actually failed. In-progress and
# successful runs are left alone; a cancelled run was stopped on purpose.
is_retryable() {
  case $1 in
    failure|timed_out|startup_failure) return 0 ;;
    *) return 1 ;;
  esac
}

printf 'Watching %s (poll %ss, max-attempts %s%s)\n\n' \
  "$tag" "$interval" "$max_attempts" "$([ "$dry_run" = true ] && echo ', dry-run')"

while true; do
  # --limit 100 comfortably covers the one-run-per-product fan-out.
  runs=$(gh run list --branch "$tag" --limit 100 \
    --json databaseId,workflowName,status,conclusion 2>/dev/null || echo '[]')

  if [ "$(jq length <<<"$runs")" -eq 0 ]; then
    echo "No runs found for $tag, waiting..."
    sleep "$interval"
    continue
  fi

  running=0 succeeded=0 retryable=0 gaveup=0 triggered=0

  while IFS=$'\t' read -r id name status conclusion; do
    if [ "$status" != completed ]; then
      running=$((running + 1))
      printf '  %-6s %s (%s)\n' wait "$name" "$status"
      continue
    fi

    if [ "$conclusion" = success ]; then
      succeeded=$((succeeded + 1))
      printf '  %-6s %s\n' ok "$name"
      continue
    fi

    # Completed but not successful. Only failures are re-run; anything else
    # (e.g. cancelled) is reported and left as-is.
    if [ -z "${gave_up[$id]:-}" ] && ! is_retryable "$conclusion"; then
      printf '  %-6s %s (%s)\n' skip "$name" "$conclusion"
      continue
    fi

    if [ -z "${attempts[$id]:-}" ]; then
      attempts[$id]=$(gh run view "$id" --json runAttempt --jq .runAttempt 2>/dev/null || echo 1)
    fi

    if [ -n "${gave_up[$id]:-}" ] || [ "${attempts[$id]}" -ge "$max_attempts" ]; then
      gave_up[$id]=1
      gaveup=$((gaveup + 1))
      printf '  %-6s %s (gave up after %s attempts: %s)\n' FAIL "$name" "${attempts[$id]}" "$conclusion"
      continue
    fi

    retryable=$((retryable + 1))
    next=$((attempts[$id] + 1))
    if [ "$dry_run" = true ]; then
      printf '  %-6s %s (would be attempt %s)\n' retry "$name" "$next"
    elif gh run rerun "$id" --failed >/dev/null 2>&1; then
      attempts[$id]=$next
      triggered=$((triggered + 1))
      printf '  %-6s %s (attempt %s)\n' retry "$name" "$next"
    else
      # A rerun is refused until the run has fully settled; the next poll retries.
      printf '  %-6s %s (rerun deferred)\n' retry "$name"
    fi
  done < <(jq -r 'sort_by(.workflowName)[] | [.databaseId, .workflowName, .status, .conclusion] | @tsv' <<<"$runs")

  printf '\n  %s | ok %s, waiting %s, retrying %s, gave up %s (triggered %s)\n\n' \
    "$(date +%H:%M:%S)" "$succeeded" "$running" "$retryable" "$gaveup" "$triggered"

  # Nothing running and nothing left to retry means we are done.
  if [ "$running" -eq 0 ] && [ "$retryable" -eq 0 ]; then
    if [ "$gaveup" -gt 0 ]; then
      echo "Done: $gaveup run(s) still failing after $max_attempts attempts. Check their logs."
      exit 1
    fi
    echo "Done: all $succeeded run(s) for $tag passed."
    exit 0
  fi

  sleep "$interval"
done
