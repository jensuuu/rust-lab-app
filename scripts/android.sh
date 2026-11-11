#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

ANDROID_TARGETS=(aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android)
OUTPUT_DIR="$PROJECT_ROOT/dist"

usage() {
  cat << USAGE
Usage: scripts/android.sh <command> [options]

Commands:
  setup              Check the Android SDK, add the Rust Android targets, create src-tauri/gen/android
  dev                Run the app on a connected phone or emulator
  build [options]    Build an APK into dist/

Build options:
  --debug            Debug APK (signed with the debug key, installs right away)
  --arm64            Only build for 64-bit ARM phones (smaller and much faster)
  --sign             Sign the release APK so it can be installed
                     Uses \$ANDROID_KEYSTORE if set, otherwise the Android debug keystore

Environment: ANDROID_HOME (default ~/Android/Sdk), NDK_HOME (default: newest NDK in the SDK), JAVA_HOME
USAGE
}

detect_android_env() {
  export ANDROID_HOME="${ANDROID_HOME:-${ANDROID_SDK_ROOT:-$HOME/Android/Sdk}}"
  [ -d "$ANDROID_HOME" ] \
    || fail "Android SDK not found at $ANDROID_HOME. Install Android Studio and set ANDROID_HOME."

  if [ -z "${NDK_HOME:-}" ]; then
    local newest=""
    newest="$(ls -1d "$ANDROID_HOME"/ndk/* 2>/dev/null | sort -V | tail -n1 || true)"
    [ -n "$newest" ] \
      || fail "Android NDK not found. In Android Studio: SDK Manager > SDK Tools > NDK (Side by side)."
    export NDK_HOME="$newest"
  fi

  have java || fail "Java not found. Install a JDK 17 or newer (Fedora: sudo dnf install java-17-openjdk-devel)."
}

add_rust_targets() {
  have rustup || fail "rustup is required to add the Android targets."
  log "Adding Rust Android targets"
  rustup target add "${ANDROID_TARGETS[@]}"
}

init_android_project() {
  if [ -d "$PROJECT_ROOT/src-tauri/gen/android" ]; then
    return
  fi

  log "Creating the Android project (src-tauri/gen/android)"
  (cd "$PROJECT_ROOT" && cargo tauri android init)
}

apply_android_icons() {
  local source="$PROJECT_ROOT/src-tauri/icons/android-adaptive"
  local target="$PROJECT_ROOT/src-tauri/gen/android/app/src/main/res"
  local file destination

  [ -d "$source" ] && [ -d "$target" ] || return 0

  log "Applying the Android launcher icons"
  while IFS= read -r file; do
    destination="$target/${file#"$source"/}"
    mkdir -p "$(dirname "$destination")"
    cp "$file" "$destination"
    rm -f "${destination%.png}.webp"
  done < <(find "$source" -type f)
}

sign_apk() {
  local unsigned="$1" output="$2"
  local tools keystore aligned

  tools="$(ls -1d "$ANDROID_HOME"/build-tools/* 2>/dev/null | sort -V | tail -n1 || true)"
  [ -n "$tools" ] || fail "Android build-tools not found. Install them in Android Studio's SDK Manager."

  aligned="$(mktemp --suffix=.apk)"
  "$tools/zipalign" -f -p 4 "$unsigned" "$aligned"

  if [ -n "${ANDROID_KEYSTORE:-}" ]; then
    log "Signing with $ANDROID_KEYSTORE (you will be asked for the password)"
    "$tools/apksigner" sign --ks "$ANDROID_KEYSTORE" --out "$output" "$aligned"
  else
    keystore="$HOME/.android/debug.keystore"

    if [ ! -f "$keystore" ]; then
      log "Creating a debug keystore at $keystore"
      mkdir -p "$(dirname "$keystore")"
      keytool -genkeypair -keystore "$keystore" -storepass android -keypass android \
        -alias androiddebugkey -keyalg RSA -keysize 2048 -validity 10000 \
        -dname "CN=Android Debug,O=Android,C=US"
    fi

    warn "Signing with the Android debug key. Fine for your own phone, not for the Play Store."
    "$tools/apksigner" sign --ks "$keystore" --ks-pass pass:android --key-pass pass:android \
      --ks-key-alias androiddebugkey --out "$output" "$aligned"
  fi

  rm -f "$aligned"
}

command_setup() {
  require_rust
  require_node
  require_tauri_cli
  detect_android_env
  add_rust_targets
  install_node_modules
  init_android_project
  apply_android_icons
  log "Android setup finished. Next: scripts/android.sh build --debug --arm64"
}

command_dev() {
  require_rust
  require_node
  require_tauri_cli
  detect_android_env
  install_node_modules
  build_css
  init_android_project
  apply_android_icons
  cd "$PROJECT_ROOT"
  exec cargo tauri android dev
}

command_build() {
  local debug=0 sign=0
  local args=(--apk)

  while [ $# -gt 0 ]; do
    case "$1" in
      --debug) debug=1; args+=(--debug) ;;
      --arm64) args+=(--target aarch64) ;;
      --sign) sign=1 ;;
      -h|--help) usage; exit 0 ;;
      *) usage; fail "Unknown option: $1" ;;
    esac
    shift
  done

  require_rust
  require_node
  require_tauri_cli
  detect_android_env
  install_node_modules
  build_css
  init_android_project
  apply_android_icons

  log "Compiling the Android app (the first build takes several minutes)"
  (cd "$PROJECT_ROOT" && cargo tauri android build "${args[@]}")

  mkdir -p "$OUTPUT_DIR"
  local apks=() apk name
  while IFS= read -r apk; do
    apks+=("$apk")
  done < <(find "$PROJECT_ROOT/src-tauri/gen/android/app/build/outputs/apk" -name '*.apk' 2>/dev/null | sort)

  [ "${#apks[@]}" -gt 0 ] || fail "No APK was produced. Check the build output above."

  local version
  version="$(node -p "require('$PROJECT_ROOT/src-tauri/tauri.conf.json').version")"

  for apk in "${apks[@]}"; do
    name="rust-academy-$version"
    [ "${#apks[@]}" -eq 1 ] || name="$name-$(basename "$(dirname "$(dirname "$apk")")")"
    [ "$debug" -eq 0 ] || name="$name-debug"

    if [ "$sign" -eq 1 ] && [ "$debug" -eq 0 ]; then
      sign_apk "$apk" "$OUTPUT_DIR/$name.apk"
    else
      cp "$apk" "$OUTPUT_DIR/$name.apk"
    fi
  done

  log "APK files in dist/:"
  (cd "$OUTPUT_DIR" && ls -lh ./*.apk)
}

case "${1:-}" in
  setup) command_setup ;;
  dev) command_dev ;;
  build) shift; command_build "$@" ;;
  -h|--help|"") usage ;;
  *) usage; fail "Unknown command: $1" ;;
esac
