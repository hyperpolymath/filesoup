# FSLint installation script for Windows PowerShell

$ErrorActionPreference = "Stop"

Write-Host "FSLint Installer" -ForegroundColor Green
Write-Host "=================" -ForegroundColor Green
Write-Host ""

# Check for Rust
$cargoExists = Get-Command cargo -ErrorAction SilentlyContinue
if (-not $cargoExists) {
    Write-Host "Rust not found. Please install Rust first:" -ForegroundColor Yellow
    Write-Host "  https://www.rust-lang.org/tools/install" -ForegroundColor Yellow
    exit 1
}

Write-Host "Rust found: $(cargo --version)"

# Installation method
Write-Host ""
Write-Host "Choose installation method:"
Write-Host "1) Install from crates.io (recommended)"
Write-Host "2) Build from source"
$choice = Read-Host "Enter choice [1-2]"

switch ($choice) {
    "1" {
        Write-Host "Installing from crates.io..." -ForegroundColor Green
        cargo install fslint
    }
    "2" {
        Write-Host "Building from source..." -ForegroundColor Green

        # Check if we're in the repo
        if (-not (Test-Path "Cargo.toml")) {
            Write-Host "Cloning repository..."
            git clone https://github.com/Hyperpolymath/file-soup.git
            Set-Location file-soup
        }

        Write-Host "Building release binary..."
        cargo build --release

        Write-Host "Installing binary..."
        $installPath = "$env:USERPROFILE\.cargo\bin"
        if (-not (Test-Path $installPath)) {
            New-Item -ItemType Directory -Path $installPath -Force
        }
        Copy-Item "target\release\fslint.exe" -Destination $installPath -Force

        Write-Host "Cleaning up..."
        cargo clean
    }
    default {
        Write-Host "Invalid choice" -ForegroundColor Red
        exit 1
    }
}

# Verify installation
$fslintExists = Get-Command fslint -ErrorAction SilentlyContinue
if ($fslintExists) {
    Write-Host ""
    Write-Host "✓ FSLint installed successfully!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Try it out:"
    Write-Host "  fslint scan ."
    Write-Host "  fslint plugins"
    Write-Host ""
    Write-Host "For more information, visit: https://github.com/Hyperpolymath/file-soup"
} else {
    Write-Host "✗ Installation failed" -ForegroundColor Red
    exit 1
}
