#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

require_rust
require_node
require_tauri_cli
install_node_modules
build_css

log "Starting Rust Academy in development mode"
cd "$PROJECT_ROOT"
exec cargo tauri dev
