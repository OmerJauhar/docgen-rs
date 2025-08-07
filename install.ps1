# DocGen Installation Script for Windows
# GreyBeard Outsourcing - Internal Tool
# Run this script as Administrator for system-wide installation

param(
    [switch]$User,
    [string]$InstallPath = "",
    [switch]$Help
)

if ($Help) {
    Write-Host "DocGen Installation Script for Windows" -ForegroundColor Cyan
    Write-Host "=====================================`n"
    Write-Host "Usage:"
    Write-Host "  .\install.ps1                    # Install system-wide (requires admin)"
    Write-Host "  .\install.ps1 -User              # Install for current user only"
    Write-Host "  .\install.ps1 -InstallPath C:\   # Custom installation path"
    Write-Host "  .\install.ps1 -Help              # Show this help"
    exit 0
}

# Function to check if running as administrator
function Test-IsAdmin {
    $currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

# Main installation function
function Install-DocGen {
    Write-Host "[*] DocGen Installation for GreyBeard Outsourcing" -ForegroundColor Green
    Write-Host "================================================`n"

    # Determine installation directory
    if ($InstallPath -ne "") {
        $installDir = $InstallPath
    } elseif ($User) {
        $installDir = "$env:LOCALAPPDATA\Programs\DocGen"
    } else {
        if (-not (Test-IsAdmin)) {
            Write-Host "[!] System-wide installation requires administrator privileges." -ForegroundColor Red
            Write-Host "   Please run PowerShell as Administrator or use -User flag." -ForegroundColor Yellow
            exit 1
        }
        $installDir = "$env:ProgramFiles\GreyBeard\DocGen"
    }

    Write-Host "[+] Installation directory: $installDir"

    # Create installation directory
    if (-not (Test-Path $installDir)) {
        New-Item -ItemType Directory -Path $installDir -Force | Out-Null
        Write-Host "[+] Created installation directory"
    }

    # Check if Rust is installed
    Write-Host "[?] Checking dependencies..."
    
    try {
        $rustVersion = cargo --version
        Write-Host "[+] Rust found: $rustVersion" -ForegroundColor Green
    } catch {
        Write-Host "[!] Rust not found. Installing Rust..." -ForegroundColor Yellow
        
        # Download and install Rustup
        $rustupUrl = "https://win.rustup.rs/x86_64"
        $rustupPath = "$env:TEMP\rustup-init.exe"
        
        Write-Host "[*] Downloading Rust installer..."
        try {
            Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupPath
            Write-Host "[*] Installing Rust..."
            Start-Process -FilePath $rustupPath -ArgumentList "-y" -Wait
            
            # Refresh environment variables
            $env:PATH = [System.Environment]::GetEnvironmentVariable("PATH", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("PATH", "User")
            
            Write-Host "[+] Rust installed successfully"
        } catch {
            Write-Host "[!] Automatic Rust installation failed." -ForegroundColor Red
            Write-Host "    Please install Rust manually from https://rustup.rs/" -ForegroundColor Yellow
            Write-Host "    Then restart PowerShell and run this script again." -ForegroundColor Yellow
            exit 1
        }
    }

    # Check if Git is installed
    try {
        $gitVersion = git --version
        Write-Host "[+] Git found: $gitVersion" -ForegroundColor Green
    } catch {
        Write-Host "[!] Git not found. Please install Git from https://git-scm.com/download/win" -ForegroundColor Red
        Write-Host "   Git is required for DocGen to analyze repositories." -ForegroundColor Yellow
        exit 1
    }

    # Build DocGen
    Write-Host "[*] Building DocGen..."
    
    if (Test-Path "Cargo.toml") {
        # Building from source
        cargo build --release
        $sourceBinary = "target\release\docgen.exe"
    } else {
        Write-Host "[!] Cargo.toml not found. Please run this script from the DocGen source directory." -ForegroundColor Red
        exit 1
    }

    # Copy binary to installation directory
    $targetBinary = "$installDir\docgen.exe"
    Copy-Item $sourceBinary $targetBinary -Force
    Write-Host "[+] Binary installed to $targetBinary"

    # Add to PATH
    Write-Host "[*] Configuring PATH..."
    
    if ($User) {
        # User-level PATH
        $userPath = [System.Environment]::GetEnvironmentVariable("PATH", "User")
        if ($userPath -notlike "*$installDir*") {
            if ($userPath) {
                $newPath = "$userPath;$installDir"
            } else {
                $newPath = $installDir
            }
            [System.Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
            Write-Host "[+] Added to user PATH"
        }
    } else {
        # System-level PATH  
        $systemPath = [System.Environment]::GetEnvironmentVariable("PATH", "Machine")
        if ($systemPath -notlike "*$installDir*") {
            if ($systemPath) {
                $newPath = "$systemPath;$installDir"
            } else {
                $newPath = $installDir
            }
            [System.Environment]::SetEnvironmentVariable("PATH", $newPath, "Machine")
            Write-Host "[+] Added to system PATH"
        }
    }

    # Create desktop shortcut (optional)
    $createShortcut = Read-Host "Create desktop shortcut? (y/n) [y]"
    if ($createShortcut -eq "" -or $createShortcut -eq "y" -or $createShortcut -eq "Y") {
        $shortcutPath = "$env:USERPROFILE\Desktop\DocGen.lnk"
        $WScriptShell = New-Object -ComObject WScript.Shell
        $shortcut = $WScriptShell.CreateShortcut($shortcutPath)
        $shortcut.TargetPath = $targetBinary
        $shortcut.Arguments = "generate"
        $shortcut.Description = "DocGen - AI Documentation Generator"
        $shortcut.WorkingDirectory = $installDir
        $shortcut.Save()
        Write-Host "[+] Desktop shortcut created"
    }

    Write-Host "`n[*] Installation completed successfully!" -ForegroundColor Green
    Write-Host "================================================"
    Write-Host "You can now run DocGen using any of these commands:"
    Write-Host "  docgen generate              # Start the documentation generator"
    Write-Host "  docgen config               # Configure user settings"
    Write-Host "  docgen version              # Show version information"
    Write-Host "`nNote: You may need to restart your terminal for PATH changes to take effect."
    Write-Host "`nFor support, contact: omer.jauhar@greybeardsupport.com"
}

# Run installation
Install-DocGen
