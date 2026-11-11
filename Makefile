.PHONY: help setup dev build binary packages check css clean distclean android-setup android-dev android-apk

help:
	@echo "make setup      Install system packages, Rust, Tauri CLI and npm packages and does setup for android apk."
	@echo "make dev        Run the app in development mode"
	@echo "make build      Build release binary and installer packages into dist/"
	@echo "make binary     Build only the release executable into dist/"
	@echo "make packages   Build rpm and deb packages into dist/"
	@echo "make check      Run rustfmt and clippy"
	@echo "make css        Build the Tailwind CSS"
	@echo "make dev    Run on a connected phone or emulator"
	@echo "make apk    Build a signed arm64 release APK into dist/"
	@echo "make clean      Remove build output"
	@echo "make distclean  Remove build output and node_modules"

setup:
	./scripts/setup.sh
	./scripts/android.sh setup

dev:
	./scripts/dev.sh

build:
	./scripts/build.sh

binary:
	./scripts/build.sh --binary

packages:
	./scripts/build.sh --bundles rpm,deb

check:
	./scripts/check.sh

css:
	yarn css:build

clean:
	./scripts/clean.sh

distclean:
	./scripts/clean.sh --deep

genclean:
	rm -rf ./src-tauri/gen/
	rm -rf ./src-tauri/target/

dev:
	./scripts/android.sh dev

apk:
	./scripts/android.sh build --arm64 --sign

icons:
	cargo tauri icon src-tauri/icons/icon.png

sharefiles:
	updog -b 0.0.0.0 -d ./dist
