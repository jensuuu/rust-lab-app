#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

require_rust

log "Checking formatting"
(cd "$PROJECT_ROOT" && cargo fmt --all -- --check)

log "Running clippy"
(cd "$PROJECT_ROOT" && cargo clippy --workspace --all-targets -- -D warnings)

log "All checks passed"
