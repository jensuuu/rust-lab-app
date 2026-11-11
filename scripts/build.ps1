$ErrorActionPreference = "Stop"

$ProjectRoot = Split-Path -Parent $PSScriptRoot
$OutputDir = Join-Path $ProjectRoot "dist"

function Require-Command($Name, $Hint) {
    if (-not (Get-Command $Name -ErrorAction SilentlyContinue)) {
        throw "$Name not found. $Hint"
    }
}

Require-Command "rustc" "Install Rust from https://rustup.rs"
Require-Command "cargo" "Install Rust from https://rustup.rs"
Require-Command "node" "Install Node.js from https://nodejs.org"
Require-Command "npm" "Install Node.js from https://nodejs.org"

cargo tauri --version *> $null
if ($LASTEXITCODE -ne 0) {
    Write-Host "Installing Tauri CLI"
    cargo install tauri-cli --version "^2" --locked
}

Set-Location $ProjectRoot

Write-Host "Installing npm dependencies"
npm install --no-audit --no-fund

Write-Host "Building Tailwind CSS"
npm run css:build

Write-Host "Compiling the app"
cargo tauri build @args

if (Test-Path $OutputDir) {
    Remove-Item -Recurse -Force $OutputDir
}
New-Item -ItemType Directory -Path $OutputDir | Out-Null

$Release = Join-Path $ProjectRoot "target\release"
Copy-Item (Join-Path $Release "rust-academy.exe") $OutputDir -ErrorAction SilentlyContinue
Get-ChildItem (Join-Path $Release "bundle") -Recurse -Include *.msi, *.exe |
    Copy-Item -Destination $OutputDir

Write-Host "Build finished. Files are in dist\"
