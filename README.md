# Rust Academy

A desktop and Android app for learning Rust, built with Tauri 2 and a Rust backend. The interface is styled with Tailwind CSS.

- **Lessons**: 12 short lessons, each with an example and an exercise that is checked by compiling and running your code.
- **The Book**: the full text of *The Rust Programming Language* (all chapters and appendices), with search, previous and next navigation, syntax-highlighted code, and Run and Copy buttons on code blocks.

## Requirements

- Rust installed through rustup (the app calls `rustc` to run your code, edition 2024 needs Rust 1.85 or newer)
- Node.js and npm (only used to build the Tailwind CSS)
- Tauri CLI: `cargo install tauri-cli --version "^2" --locked`
- Tauri system libraries for your OS: https://v2.tauri.app/start/prerequisites/

On Fedora:

```
sudo dnf install webkit2gtk4.1-devel openssl-devel curl wget file libappindicator-gtk3-devel librsvg2-devel libxdo-devel
sudo dnf group install "c-development"
```

## Scripts

The `scripts` folder and the `Makefile` wrap the whole workflow.

| Command | What it does |
| --- | --- |
| `./scripts/setup.sh` or `make setup` | Installs system packages (dnf, apt or pacman), Rust, Tauri CLI and npm packages |
| `./scripts/dev.sh` or `make dev` | Builds the CSS and starts the app in development mode |
| `./scripts/build.sh` or `make build` | Full release build, with the results copied into `dist/` |
| `./scripts/build.sh --binary` or `make binary` | Release executable only, no installers |
| `./scripts/build.sh --bundles rpm,deb` or `make packages` | Only the chosen package types |
| `./scripts/check.sh` or `make check` | Runs rustfmt and clippy |
| `./scripts/clean.sh` or `make clean` | Removes build output (`--deep` also removes node_modules) |
| `scripts\build.ps1` | Release build on Windows (PowerShell) |
| `./scripts/android.sh` | Android setup, dev run and APK build (see Android) |

The build script checks that Rust 1.85 or newer, Node.js and the Tauri CLI are installed and tells you what is missing.

First time on a new machine:

```
make setup
make build
```

Your installers and the executable end up in `dist/`.

## Android

The same project builds an Android APK. The layout switches to a phone layout (slide-out menu, symbol bar above the code editor).

**Running code on Android:** a phone has no Rust compiler, so on Android the Run and Check buttons send your code to the [Rust Playground](https://play.rust-lang.org) (stable, edition 2024). This needs an internet connection and only supports the crates the Playground offers. Lessons and the book text work offline. On desktop, code still runs with your local `rustc`.

Requirements, on top of the ones above:

- Android Studio (or the command line tools) with **Android SDK Platform**, **Build-Tools** and **NDK (Side by side)** installed from the SDK Manager
- JDK 17 or newer (Fedora: `sudo dnf install java-17-openjdk-devel`)
- `ANDROID_HOME` pointing at the SDK (default `~/Android/Sdk`; the NDK is found automatically)

```
make android-setup                     # Rust Android targets + src-tauri/gen/android
./scripts/android.sh dev               # run on a connected phone or emulator
./scripts/android.sh build --debug --arm64   # quick APK that installs right away
make android-apk                       # signed release APK for 64-bit ARM phones
```

APKs end up in `dist/`. Install one with `adb install dist/<file>.apk` or copy it to the phone and open it (allow installs from your file manager). Release builds with `--sign` use the Android debug key unless `ANDROID_KEYSTORE` points at your own keystore.

The Android launcher icons (adaptive icon: gear on a dark background, safe for round and rounded-square launchers) live in `src-tauri/icons/android-adaptive/` and are copied into the Android project automatically on every setup, dev and build. `cargo tauri icon` only creates the desktop icons from `icon.png`; it does not replace these.

## Languages

The interface and lesson texts are available in English and Estonian. The language switcher (EN / ET) is at the top of the sidebar, the choice is remembered, and the first start follows the system language.

Translating works by wrapping, so no text has to be rewritten:

- In HTML, add `data-i18n` to an element (or `data-i18n-placeholder`, `data-i18n-aria-label`, `data-i18n-title` for attributes).
- In JavaScript, wrap the text in `t("English text")`. Placeholders work too: `t("{done} of {total} done", { done, total })`.
- Add the English text and its translation to `src/locales/et.js` (interface) or `src/locales/et.lessons.js` (lessons).

Text without a translation is shown in English, so nothing breaks while a translation is missing. New lessons added in `lessons.rs` show in English until their texts are added to `et.lessons.js`; the keys are the English texts exactly as written in `lessons.rs`.

To add another language, create `src/locales/<code>.js` with `I18n.add("<code>", { ... })`, add its `<script>` tag to `src/index.html` and add the code to `LANGUAGES` in `src/i18n.js`. The switcher shows it automatically.

Not translated: code, starter code and expected output in the lessons, compiler messages, and the text of The Book (it stays English, with a note shown in Estonian mode).

## Project layout

```
Cargo.toml              Workspace and build profiles
package.json            Tailwind CSS build scripts
Makefile                Shortcuts for the scripts
scripts/                Setup, dev, build, check and clean scripts
ui/input.css            Tailwind entry file (builds to src/styles.css)
rust-toolchain.toml     Pinned toolchain channel and components
.cargo/config.toml      Cargo behavior
src/                    Frontend (index.html, app.js, i18n.js, generated styles.css)
src/locales/            Translations (et.js for the interface, et.lessons.js for lessons)
src-tauri/data/         book.json, the book text split into chapters and sections
src-tauri/src/
  main.rs               Desktop entry point
  lib.rs                Shared app setup and the Android entry point
  commands.rs           Commands called from the frontend
  lessons.rs            Lesson and exercise content
  book.rs               Loads the embedded book, serves sections, searches text
  runner.rs             Run result type, picks the local or remote runner
  local.rs              Desktop: compiles and runs user code with rustc and time limits
  remote.rs             Android: runs user code through the Rust Playground
  progress.rs           Saves completed exercises
```

## Notes about the book

- The book text is embedded in the app binary, so it works fully offline.
- Diagrams from the printed book are not included, and a short placeholder marks where each one appears.
- Run buttons appear only on complete programs with a `main` function. Snippets that use external crates or `--snip--` are left read-only.
- The Rust book is by Steve Klabnik, Carol Nichols, Chris Krycho and the Rust community, and is available under the MIT and Apache 2.0 licenses.

## Adding a lesson

Add another `Lesson` entry to the `LESSONS` list in `src-tauri/src/lessons.rs` with the next free `id`. The sidebar updates automatically.

## Build configuration

- Release builds use fat LTO, a single codegen unit, `panic = "abort"`, stripped symbols and size optimization for a small, fast binary.
- Dev builds compile dependencies with optimizations while keeping your own code quick to rebuild.
- Build scripts and proc macros are compiled with minimal optimization so full builds finish sooner.
