#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

install_system_packages() {
  if have dnf; then
    log "Installing system packages with dnf"
    sudo dnf install -y webkit2gtk4.1-devel openssl-devel curl wget file \
      libappindicator-gtk3-devel librsvg2-devel libxdo-devel nodejs npm rpm-build
    sudo dnf group install -y "c-development"
  elif have apt-get; then
    log "Installing system packages with apt"
    sudo apt-get update
    sudo apt-get install -y libwebkit2gtk-4.1-dev build-essential curl wget file \
      libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev nodejs npm
  elif have pacman; then
    log "Installing system packages with pacman"
    sudo pacman -S --needed --noconfirm webkit2gtk-4.1 base-devel curl wget file openssl \
      xdotool libappindicator-gtk3 librsvg nodejs npm
  else
    warn "Unknown package manager. Install the Tauri prerequisites manually: https://v2.tauri.app/start/prerequisites/"
  fi
}

install_rust() {
  if have rustup; then
    log "Updating Rust"
    rustup update stable
    return
  fi

  if have rustc; then
    return
  fi

  log "Installing Rust with rustup"
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source "$HOME/.cargo/env"
}

install_tauri_cli() {
  if cargo tauri --version >/dev/null 2>&1; then
    return
  fi

  log "Installing Tauri CLI"
  cargo install tauri-cli --version "^2" --locked
}

install_system_packages
install_rust
require_rust
install_tauri_cli
require_node
install_node_modules

log "Setup finished. Next: scripts/build.sh"
