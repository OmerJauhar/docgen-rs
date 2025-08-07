# DocGen Installation Script with Robust Rust Installation
# GreyBeard Outsourcing - Internal Tool

param(
    [switch]$User = $false,
    [switch]$Help = $false
)

function Show-Help {
    Write-Host "DocGen Installation Script with Auto Rust Installation"
    Write-Host "====================================================="
    Write-Host "Usage:"
    Write-Host "  .\install-rust-auto.ps1         # Install system-wide (requires Admin)"
    Write-Host "  .\install-rust-auto.ps1 -User   # Install for current user only"
    Write-Host "  .\install-rust-auto.ps1 -Help   # Show this help"
}

function Test-RustInstallation {
    try {
        $version = & cargo --version 2>$null
        if ($version) {
            Write-Host "[+] Rust found: $version" -ForegroundColor Green
            return $true
        }
    } catch {}
    return $false
}

function Install-RustAutomatically {
    Write-Host "[*] Installing Rust automatically..." -ForegroundColor Yellow
    
    # Method 1: Try using Windows Package Manager (winget) if available
    try {
        Write-Host "[?] Checking for Windows Package Manager..."
        $wingetVersion = & winget --version 2>$null
        if ($wingetVersion) {
            Write-Host "[+] Found winget: $wingetVersion"
            Write-Host "[*] Installing Rust via winget..."
            & winget install -e --id Rustlang.Rustup --silent --accept-source-agreements --accept-package-agreements
            
            # Refresh PATH
            $env:PATH = [System.Environment]::GetEnvironmentVariable("PATH", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("PATH", "User")
            
            if (Test-RustInstallation) {
                Write-Host "[+] Rust installed successfully via winget!" -ForegroundColor Green
                return $true
            }
        }
    } catch {
        Write-Host "[!] winget installation failed, trying direct download..." -ForegroundColor Yellow
    }
    
    # Method 2: Direct download and install
    try {
        Write-Host "[*] Downloading Rust installer directly..."
        
        # Set execution policy temporarily
        $currentPolicy = Get-ExecutionPolicy
        Set-ExecutionPolicy -ExecutionPolicy Bypass -Scope Process -Force
        
        # Download rustup
        $rustupUrl = "https://win.rustup.rs/x86_64"
        $rustupPath = "$env:TEMP\rustup-init.exe"
        
        # Use .NET WebClient for more reliable download
        $webClient = New-Object System.Net.WebClient
        $webClient.DownloadFile($rustupUrl, $rustupPath)
        
        if (Test-Path $rustupPath) {
            Write-Host "[+] Downloaded Rust installer successfully"
            Write-Host "[*] Installing Rust (this may take a few minutes)..."
            
            # Install Rust with verbose output
            $process = Start-Process -FilePath $rustupPath -ArgumentList "--default-host", "x86_64-pc-windows-msvc", "--default-toolchain", "stable", "-y" -Wait -PassThru
            
            if ($process.ExitCode -eq 0) {
                Write-Host "[+] Rust installation completed"
                
                # Refresh environment variables multiple ways
                $env:PATH = [System.Environment]::GetEnvironmentVariable("PATH", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("PATH", "User")
                $env:CARGO_HOME = [System.Environment]::GetEnvironmentVariable("CARGO_HOME", "User")
                $env:RUSTUP_HOME = [System.Environment]::GetEnvironmentVariable("RUSTUP_HOME", "User")
                
                # Add common Rust paths manually
                $commonRustPaths = @(
                    "$env:USERPROFILE\.cargo\bin",
                    "$env:LOCALAPPDATA\.cargo\bin",
                    "$env:APPDATA\.cargo\bin"
                )
                
                foreach ($path in $commonRustPaths) {
                    if (Test-Path $path) {
                        $env:PATH += ";$path"
                        Write-Host "[+] Added to PATH: $path"
                    }
                }
                
                # Test installation
                if (Test-RustInstallation) {
                    Write-Host "[+] Rust installed and verified successfully!" -ForegroundColor Green
                    return $true
                } else {
                    Write-Host "[!] Rust installed but not detected. You may need to restart PowerShell." -ForegroundColor Yellow
                    Write-Host "    Manual PATH addition may be required." -ForegroundColor Yellow
                }
            }
        }
    } catch {
        Write-Host "[!] Automatic Rust installation failed: $_" -ForegroundColor Red
    }
    
    return $false
}

function Install-DocGen {
    Write-Host "[*] DocGen Installation for GreyBeard Outsourcing" -ForegroundColor Green
    Write-Host "================================================"

    # Check for admin rights if system-wide installation
    if (-not $User) {
        $isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")
        if (-not $isAdmin) {
            Write-Host "[!] System-wide installation requires administrator privileges." -ForegroundColor Red
            Write-Host "    Please run PowerShell as Administrator or use -User flag." -ForegroundColor Yellow
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

    # Check and install dependencies
    Write-Host "[?] Checking dependencies..."
    
    # Check Rust with auto-installation
    if (-not (Test-RustInstallation)) {
        Write-Host "[!] Rust not found. Attempting automatic installation..." -ForegroundColor Yellow
        
        if (-not (Install-RustAutomatically)) {
            Write-Host ""
            Write-Host "[!] Automatic Rust installation failed." -ForegroundColor Red
            Write-Host "    Please install Rust manually:" -ForegroundColor Yellow
            Write-Host "    1. Go to https://rustup.rs/" -ForegroundColor Cyan
            Write-Host "    2. Download and run the installer" -ForegroundColor Cyan
            Write-Host "    3. Restart PowerShell" -ForegroundColor Cyan
            Write-Host "    4. Run this script again" -ForegroundColor Cyan
            Write-Host ""
            Read-Host "Press Enter to continue anyway (will fail) or Ctrl+C to exit"
        }
    }

    # Check Git
    $gitVersion = & git --version 2>$null
    if ($gitVersion) {
        Write-Host "[+] Git found: $gitVersion" -ForegroundColor Green
    } else {
        Write-Host "[!] Git not found. Please install Git from https://git-scm.com/download/win" -ForegroundColor Red
        Write-Host "    Git is required for DocGen to work properly." -ForegroundColor Yellow
        Read-Host "Press Enter to continue anyway or Ctrl+C to exit"
    }

    # Build DocGen
    Write-Host "[*] Building DocGen..."
    
    if (-not (Test-Path "Cargo.toml")) {
        Write-Host "[!] Cargo.toml not found. Please run this script from the DocGen source directory." -ForegroundColor Red
        exit 1
    }

    try {
        & cargo build --release
        if ($LASTEXITCODE -eq 0) {
            Write-Host "[+] Build completed successfully" -ForegroundColor Green
        } else {
            Write-Host "[!] Build failed with exit code $LASTEXITCODE" -ForegroundColor Red
            exit 1
        }
    } catch {
        Write-Host "[!] Build failed: $_" -ForegroundColor Red
        Write-Host "    Make sure Rust and Cargo are properly installed." -ForegroundColor Yellow
        exit 1
    }

    # Install binary
    $sourceBinary = "target\release\docgen.exe"
    $targetBinary = "$installDir\docgen.exe"
    
    if (Test-Path $sourceBinary) {
        Copy-Item $sourceBinary $targetBinary -Force
        Write-Host "[+] Binary installed to $targetBinary"
    } else {
        Write-Host "[!] Binary not found at $sourceBinary" -ForegroundColor Red
        exit 1
    }

    # Add to PATH
    Write-Host "[*] Configuring PATH..."
    try {
        if ($User) {
            $currentPath = [System.Environment]::GetEnvironmentVariable("PATH", "User")
            if ($currentPath -notlike "*$installDir*") {
                $newPath = if ($currentPath) { "$currentPath;$installDir" } else { $installDir }
                [System.Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
                Write-Host "[+] Added to user PATH"
            }
        } else {
            $currentPath = [System.Environment]::GetEnvironmentVariable("PATH", "Machine")
            if ($currentPath -notlike "*$installDir*") {
                $newPath = if ($currentPath) { "$currentPath;$installDir" } else { $installDir }
                [System.Environment]::SetEnvironmentVariable("PATH", $newPath, "Machine")
                Write-Host "[+] Added to system PATH"
            }
        }
    } catch {
        Write-Host "[!] Failed to update PATH automatically" -ForegroundColor Yellow
        Write-Host "    Please add manually: $installDir" -ForegroundColor Cyan
    }

    Write-Host ""
    Write-Host "[*] Installation completed successfully!" -ForegroundColor Green
    Write-Host "================================================"
    Write-Host "To test DocGen, restart PowerShell and run:"
    Write-Host "docgen version" -ForegroundColor Green
    Write-Host ""
    Write-Host "For support, contact: omer.jauhar@greybeardsupport.com"
}

if ($Help) {
    Show-Help
} else {
    Install-DocGen
}
