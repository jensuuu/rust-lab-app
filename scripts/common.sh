#!/usr/bin/env bash

MIN_RUST_VERSION="1.85.0"
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

log() {
  printf '\033[1;34m==>\033[0m %s\n' "$*"
}

warn() {
  printf '\033[1;33mwarning:\033[0m %s\n' "$*" >&2
}

fail() {
  printf '\033[1;31merror:\033[0m %s\n' "$*" >&2
  exit 1
}

have() {
  command -v "$1" >/dev/null 2>&1
}

version_at_least() {
  [ "$(printf '%s\n%s\n' "$2" "$1" | sort -V | head -n1)" = "$2" ]
}

require_rust() {
  have rustc || fail "rustc not found. Run scripts/setup.sh or install Rust from https://rustup.rs"
  have cargo || fail "cargo not found. Run scripts/setup.sh or install Rust from https://rustup.rs"

  local installed
  installed="$(rustc --version | awk '{print $2}')"
  version_at_least "$installed" "$MIN_RUST_VERSION" \
    || fail "Rust $MIN_RUST_VERSION or newer is required (found $installed). Run: rustup update stable"
}

require_node() {
  have node || fail "Node.js not found. Run scripts/setup.sh or install nodejs and npm"
  have npm || fail "npm not found. Run scripts/setup.sh or install nodejs and npm"
}

require_tauri_cli() {
  cargo tauri --version >/dev/null 2>&1 \
    || fail "Tauri CLI not found. Run scripts/setup.sh or: cargo install tauri-cli --version \"^2\" --locked"
}

install_node_modules() {
  if [ ! -d "$PROJECT_ROOT/node_modules" ] || [ "$PROJECT_ROOT/package.json" -nt "$PROJECT_ROOT/node_modules" ]; then
    log "Installing npm dependencies"
    (cd "$PROJECT_ROOT" && npm install --no-audit --no-fund)
    touch "$PROJECT_ROOT/node_modules"
  fi
}

build_css() {
  log "Building Tailwind CSS"
  (cd "$PROJECT_ROOT" && npm run --silent css:build)
}
