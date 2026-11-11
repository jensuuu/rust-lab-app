#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

OUTPUT_DIR="$PROJECT_ROOT/dist"
BUNDLES=""
BUILD_ARGS=()
BINARY_ONLY=0
KEEP_DIST=0

usage() {
  cat << USAGE
Usage: scripts/build.sh [options]

Options:
  --binary           Build only the executable, without installer packages
  --bundles LIST     Comma separated bundle types, for example: rpm,deb,appimage
  --debug            Build a debug version instead of release
  --keep-dist        Do not empty the dist folder before copying results
  -h, --help         Show this help
USAGE
}

while [ $# -gt 0 ]; do
  case "$1" in
    --binary) BINARY_ONLY=1 ;;
    --bundles) BUNDLES="${2:?--bundles needs a value}"; shift ;;
    --debug) BUILD_ARGS+=("--debug") ;;
    --keep-dist) KEEP_DIST=1 ;;
    -h|--help) usage; exit 0 ;;
    *) usage; fail "Unknown option: $1" ;;
  esac
  shift
done

collect_artifacts() {
  local profile="release"
  for arg in "${BUILD_ARGS[@]:-}"; do
    [ "$arg" = "--debug" ] && profile="debug"
  done

  local target_dir="$PROJECT_ROOT/target/$profile"

  [ "$KEEP_DIST" -eq 1 ] || rm -rf "$OUTPUT_DIR"
  mkdir -p "$OUTPUT_DIR"

  if [ -f "$target_dir/rust-academy" ]; then
    cp "$target_dir/rust-academy" "$OUTPUT_DIR/"
  fi

  if [ -d "$target_dir/bundle" ]; then
    find "$target_dir/bundle" -type f \
      \( -name '*.rpm' -o -name '*.deb' -o -name '*.AppImage' -o -name '*.dmg' -o -name '*.msi' -o -name '*.exe' \) \
      -exec cp {} "$OUTPUT_DIR/" \;
  fi
}

print_summary() {
  log "Build finished. Files in dist/:"
  (cd "$OUTPUT_DIR" && ls -lh)
}

require_rust
require_node
require_tauri_cli
install_node_modules
build_css

if [ "$BINARY_ONLY" -eq 1 ]; then
  BUILD_ARGS+=("--no-bundle")
fi

if [ -n "$BUNDLES" ]; then
  BUILD_ARGS+=("--bundles" "$BUNDLES")
fi

log "Compiling the app (the first build takes a few minutes)"
(cd "$PROJECT_ROOT" && cargo tauri build "${BUILD_ARGS[@]}")

collect_artifacts
print_summary
