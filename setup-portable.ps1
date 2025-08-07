# DocGen Portable Setup Script
# GreyBeard Outsourcing - No Installation Required

param(
    [string]$TargetDir = "$env:USERPROFILE\DocGen-Portable",
    [switch]$Help
)

if ($Help) {
    Write-Host "DocGen Portable Setup - No Installation Required"
    Write-Host "==============================================="
    Write-Host "Usage:"
    Write-Host "  .\setup-portable.ps1                    # Setup in default location"
    Write-Host "  .\setup-portable.ps1 -TargetDir C:\Tools\DocGen  # Custom location"
    exit 0
}

function Setup-Portable {
    Write-Host "[*] DocGen Portable Setup for GreyBeard Outsourcing" -ForegroundColor Green
    Write-Host "================================================"
    
    # Check dependencies
    Write-Host "[?] Checking dependencies..."
    
    $rustVersion = & cargo --version 2>$null
    if (-not $rustVersion) {
        Write-Host "[!] Rust not found. Please install Rust from https://rustup.rs/" -ForegroundColor Red
        exit 1
    }
    Write-Host "[+] Rust found: $rustVersion" -ForegroundColor Green
    
    # Build
    Write-Host "[*] Building DocGen..."
    if (-not (Test-Path "Cargo.toml")) {
        Write-Host "[!] Cargo.toml not found. Please run this script from the DocGen source directory." -ForegroundColor Red
        exit 1
    }
    
    & cargo build --release
    if ($LASTEXITCODE -ne 0) {
        Write-Host "[!] Build failed." -ForegroundColor Red
        exit 1
    }
    
    # Create portable directory
    Write-Host "[+] Creating portable directory: $TargetDir"
    if (-not (Test-Path $TargetDir)) {
        New-Item -ItemType Directory -Path $TargetDir -Force | Out-Null
    }
    
    # Copy files
    Copy-Item "target\release\docgen.exe" "$TargetDir\docgen.exe" -Force
    Write-Host "[+] DocGen copied to portable directory"
    
    # Create batch file for easy access
    $batchContent = @"
@echo off
REM DocGen Portable Launcher
REM GreyBeard Outsourcing

"%~dp0docgen.exe" %*
"@
    $batchContent | Out-File -FilePath "$TargetDir\docgen.bat" -Encoding ASCII
    Write-Host "[+] Created launcher batch file"
    
    # Create PowerShell profile addition
    $profileAddition = @"

# DocGen Portable - GreyBeard Outsourcing
# Add this to your PowerShell profile for easy access
`$env:PATH += ";$TargetDir"
"@
    
    Write-Host ""
    Write-Host "[*] Portable setup completed!" -ForegroundColor Green
    Write-Host "============================================="
    Write-Host "DocGen is now available in: $TargetDir" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "To use DocGen:"
    Write-Host "1. Run directly: $TargetDir\docgen.exe version" -ForegroundColor Cyan
    Write-Host "2. Or add to PATH temporarily:" -ForegroundColor Cyan
    Write-Host "   `$env:PATH += ';$TargetDir'" -ForegroundColor Gray
    Write-Host "3. Or add to your PowerShell profile permanently:" -ForegroundColor Cyan
    Write-Host $profileAddition -ForegroundColor Gray
    Write-Host ""
    Write-Host "Test with: docgen version" -ForegroundColor Green
}

Setup-Portable
