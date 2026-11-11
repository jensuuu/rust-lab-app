#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

DEEP=0

if [ "${1:-}" = "--deep" ]; then
  DEEP=1
fi

log "Removing build output"
rm -rf "$PROJECT_ROOT/dist" "$PROJECT_ROOT/src/styles.css" "$PROJECT_ROOT/src-tauri/gen/schemas" \
  "$PROJECT_ROOT/src-tauri/gen/android/app/build" "$PROJECT_ROOT/src-tauri/gen/android/build" "$PROJECT_ROOT/src-tauri/gen/android/.gradle"

if have cargo; then
  (cd "$PROJECT_ROOT" && cargo clean)
fi

if [ "$DEEP" -eq 1 ]; then
  log "Removing node_modules"
  rm -rf "$PROJECT_ROOT/node_modules"
fi

log "Clean finished"
