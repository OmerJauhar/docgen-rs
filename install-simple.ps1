# DocGen Installation Script for Windows - Simplified Version
# GreyBeard Outsourcing - Internal Tool

param(
    [switch]$User = $false,
    [switch]$Help = $false
)

function Show-Help {
    Write-Host "DocGen Installation Script"
    Write-Host "=========================="
    Write-Host "Usage:"
    Write-Host "  .\install-simple.ps1         # Install system-wide (requires Admin)"
    Write-Host "  .\install-simple.ps1 -User   # Install for current user only"
    Write-Host "  .\install-simple.ps1 -Help   # Show this help"
}

function Install-DocGen {
    Write-Host "[*] DocGen Installation for GreyBeard Outsourcing" -ForegroundColor Green
    Write-Host "================================================"

    # Check for admin rights if system-wide installation
    if (-not $User) {
        $isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")
        if (-not $isAdmin) {
            Write-Host "[!] System-wide installation requires administrator privileges." -ForegroundColor Red
            Write-Host "    Please run PowerShell as Administrator or use -User flag for user installation." -ForegroundColor Yellow
            exit 1
        }
    }

    # Set installation directory
    if ($User) {
        $installDir = "$env:LOCALAPPDATA\GreyBeard\DocGen"
    } else {
        $installDir = "$env:ProgramFiles\GreyBeard\DocGen"
    }

    Write-Host "[+] Installation directory: $installDir"

    # Create installation directory
    if (-not (Test-Path $installDir)) {
        New-Item -ItemType Directory -Path $installDir -Force | Out-Null
        Write-Host "[+] Created installation directory"
    }

    # Check dependencies
    Write-Host "[?] Checking dependencies..."
    
    # Check Rust
    $rustVersion = & cargo --version 2>$null
    if ($rustVersion) {
        Write-Host "[+] Rust found: $rustVersion" -ForegroundColor Green
    } else {
        Write-Host "[!] Rust not found. Please install Rust from https://rustup.rs/" -ForegroundColor Red
        Write-Host "    After installing Rust, restart PowerShell and run this script again."
        exit 1
    }

    # Check Git
    $gitVersion = & git --version 2>$null
    if ($gitVersion) {
        Write-Host "[+] Git found: $gitVersion" -ForegroundColor Green
    } else {
        Write-Host "[!] Git not found. Please install Git from https://git-scm.com/download/win" -ForegroundColor Red
        exit 1
    }

    # Build DocGen
    Write-Host "[*] Building DocGen..."
    
    if (-not (Test-Path "Cargo.toml")) {
        Write-Host "[!] Cargo.toml not found. Please run this script from the DocGen source directory." -ForegroundColor Red
        exit 1
    }

    & cargo build --release
    if ($LASTEXITCODE -ne 0) {
        Write-Host "[!] Build failed. Please check the error messages above." -ForegroundColor Red
        exit 1
    }

    # Install binary
    $sourceBinary = "target\release\docgen.exe"
    $targetBinary = "$installDir\docgen.exe"
    
    Copy-Item $sourceBinary $targetBinary -Force
    Write-Host "[+] Binary installed to $targetBinary"

    # Manual PATH instructions
    Write-Host ""
    Write-Host "[*] Installation completed!" -ForegroundColor Green
    Write-Host "================================================"
    Write-Host "To use DocGen, add the following directory to your PATH:"
    Write-Host "$installDir" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "You can add it manually or run this command:"
    if ($User) {
        Write-Host "setx PATH `"%PATH%;$installDir`"" -ForegroundColor Cyan
    } else {
        Write-Host "setx PATH `"%PATH%;$installDir`" /M" -ForegroundColor Cyan
    }
    Write-Host ""
    Write-Host "After adding to PATH, restart PowerShell and test with:"
    Write-Host "docgen version" -ForegroundColor Green
    Write-Host ""
    Write-Host "For support, contact: omer.jauhar@greybeardsupport.com"
}

if ($Help) {
    Show-Help
} else {
    Install-DocGen
}
