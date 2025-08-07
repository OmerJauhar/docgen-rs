# DocGen Company Deployment Script
# GreyBeard Outsourcing - IT Department
# This script deploys DocGen to multiple machines in the company network

param(
    [string[]]$ComputerNames = @(),
    [string]$ComputerListFile = "",
    [string]$BinaryPath = "",
    [switch]$TestMode,
    [switch]$Force,
    [switch]$Help
)

if ($Help) {
    Write-Host "DocGen Company Deployment Script" -ForegroundColor Cyan
    Write-Host "================================`n"
    Write-Host "Usage:"
    Write-Host "  .\deploy.ps1 -ComputerNames PC1,PC2,PC3"
    Write-Host "  .\deploy.ps1 -ComputerListFile computers.txt"
    Write-Host "  .\deploy.ps1 -BinaryPath \\server\share\docgen.exe -ComputerNames PC1"
    Write-Host "  .\deploy.ps1 -TestMode -ComputerNames PC1    # Test without installing"
    Write-Host "  .\deploy.ps1 -Force -ComputerNames PC1       # Force reinstall"
    exit 0
}

# Configuration
$COMPANY_NAME = "GreyBeard Outsourcing"
$TOOL_NAME = "DocGen"
$INSTALL_PATH = "C:\Program Files\GreyBeard\DocGen"
$BINARY_NAME = "docgen.exe"
$LOG_PATH = ".\deployment-$(Get-Date -Format 'yyyyMMdd-HHmmss').log"

# Function to write log
function Write-Log {
    param($Message, $Level = "INFO")
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $logEntry = "[$timestamp] [$Level] $Message"
    Write-Host $logEntry
    Add-Content -Path $LOG_PATH -Value $logEntry
}

# Function to test remote connectivity
function Test-RemoteConnectivity {
    param($ComputerName)
    
    try {
        $result = Test-Connection -ComputerName $ComputerName -Count 1 -Quiet
        if ($result) {
            # Test WinRM connectivity
            $session = New-PSSession -ComputerName $ComputerName -ErrorAction SilentlyContinue
            if ($session) {
                Remove-PSSession $session
                return $true
            }
        }
        return $false
    } catch {
        return $false
    }
}

# Function to deploy to a single computer
function Deploy-ToComputer {
    param($ComputerName)
    
    Write-Log "Starting deployment to $ComputerName" "INFO"
    
    # Test connectivity
    if (-not (Test-RemoteConnectivity $ComputerName)) {
        Write-Log "Cannot connect to $ComputerName. Skipping." "ERROR"
        return $false
    }
    
    try {
        $session = New-PSSession -ComputerName $ComputerName
        
        # Check if already installed
        $isInstalled = Invoke-Command -Session $session -ScriptBlock {
            param($InstallPath, $BinaryName)
            Test-Path "$InstallPath\$BinaryName"
        } -ArgumentList $INSTALL_PATH, $BINARY_NAME
        
        if ($isInstalled -and -not $Force) {
            Write-Log "$TOOL_NAME already installed on $ComputerName. Use -Force to reinstall." "WARN"
            Remove-PSSession $session
            return $true
        }
        
        if ($TestMode) {
            Write-Log "TEST MODE: Would deploy to $ComputerName" "INFO"
            Remove-PSSession $session
            return $true
        }
        
        # Create installation directory
        Invoke-Command -Session $session -ScriptBlock {
            param($InstallPath)
            if (-not (Test-Path $InstallPath)) {
                New-Item -ItemType Directory -Path $InstallPath -Force | Out-Null
            }
        } -ArgumentList $INSTALL_PATH
        
        # Copy binary
        $sourcePath = if ($BinaryPath) { $BinaryPath } else { ".\target\release\$BINARY_NAME" }
        $destinationPath = "$INSTALL_PATH\$BINARY_NAME"
        
        Copy-Item -Path $sourcePath -Destination $destinationPath -ToSession $session -Force
        
        # Update system PATH
        Invoke-Command -Session $session -ScriptBlock {
            param($InstallPath)
            $currentPath = [System.Environment]::GetEnvironmentVariable("PATH", "Machine")
            if ($currentPath -notlike "*$InstallPath*") {
                $newPath = "$currentPath;$InstallPath"
                [System.Environment]::SetEnvironmentVariable("PATH", $newPath, "Machine")
            }
        } -ArgumentList $INSTALL_PATH
        
        # Create Start Menu shortcut
        Invoke-Command -Session $session -ScriptBlock {
            param($InstallPath, $BinaryName)
            $startMenuPath = "$env:ProgramData\Microsoft\Windows\Start Menu\Programs"
            $shortcutPath = "$startMenuPath\DocGen.lnk"
            
            $WScriptShell = New-Object -ComObject WScript.Shell
            $shortcut = $WScriptShell.CreateShortcut($shortcutPath)
            $shortcut.TargetPath = "$InstallPath\$BinaryName"
            $shortcut.Arguments = "generate"
            $shortcut.Description = "DocGen - AI Documentation Generator"
            $shortcut.WorkingDirectory = $InstallPath
            $shortcut.Save()
        } -ArgumentList $INSTALL_PATH, $BINARY_NAME
        
        # Verify installation
        $verification = Invoke-Command -Session $session -ScriptBlock {
            param($InstallPath, $BinaryName)
            $binaryPath = "$InstallPath\$BinaryName"
            if (Test-Path $binaryPath) {
                try {
                    $version = & $binaryPath version 2>&1
                    return "SUCCESS: $version"
                } catch {
                    return "ERROR: Binary exists but cannot execute"
                }
            } else {
                return "ERROR: Binary not found"
            }
        } -ArgumentList $INSTALL_PATH, $BINARY_NAME
        
        Remove-PSSession $session
        
        if ($verification -like "SUCCESS:*") {
            Write-Log "Successfully deployed to $ComputerName - $verification" "SUCCESS"
            return $true
        } else {
            Write-Log "Deployment verification failed on $ComputerName - $verification" "ERROR"
            return $false
        }
        
    } catch {
        Write-Log "Deployment failed for $ComputerName - $($_.Exception.Message)" "ERROR"
        if ($session) { Remove-PSSession $session }
        return $false
    }
}

# Main deployment function
function Start-Deployment {
    Write-Log "Starting $COMPANY_NAME $TOOL_NAME deployment" "INFO"
    Write-Log "Log file: $LOG_PATH" "INFO"
    
    # Determine target computers
    $targets = @()
    
    if ($ComputerListFile -and (Test-Path $ComputerListFile)) {
        $targets = Get-Content $ComputerListFile | Where-Object { $_.Trim() -ne "" }
        Write-Log "Loaded $(targets.Count) computers from $ComputerListFile" "INFO"
    } elseif ($ComputerNames.Count -gt 0) {
        $targets = $ComputerNames
        Write-Log "Deploying to $($targets.Count) specified computers" "INFO"
    } else {
        Write-Log "No target computers specified. Use -ComputerNames or -ComputerListFile" "ERROR"
        exit 1
    }
    
    # Verify binary exists
    $sourceBinary = if ($BinaryPath) { $BinaryPath } else { ".\target\release\$BINARY_NAME" }
    if (-not (Test-Path $sourceBinary)) {
        Write-Log "Binary not found: $sourceBinary" "ERROR"
        Write-Log "Please build the project first: cargo build --release" "ERROR"
        exit 1
    }
    
    Write-Log "Using binary: $sourceBinary" "INFO"
    
    # Deploy to each computer
    $successCount = 0
    $failureCount = 0
    
    foreach ($computer in $targets) {
        $computer = $computer.Trim()
        if ($computer -eq "") { continue }
        
        if (Deploy-ToComputer $computer) {
            $successCount++
        } else {
            $failureCount++
        }
    }
    
    # Summary
    Write-Log "Deployment completed" "INFO"
    Write-Log "Successful: $successCount" "INFO"
    Write-Log "Failed: $failureCount" "INFO"
    
    if ($failureCount -eq 0) {
        Write-Host "`n🎉 All deployments successful!" -ForegroundColor Green
    } else {
        Write-Host "`n⚠️ Some deployments failed. Check the log for details." -ForegroundColor Yellow
    }
    
    Write-Host "Deployment log: $LOG_PATH"
}

# Check prerequisites
if (-not (Get-Command New-PSSession -ErrorAction SilentlyContinue)) {
    Write-Host "PowerShell Remoting is required for deployment." -ForegroundColor Red
    Write-Host "Please ensure WinRM is configured on target machines." -ForegroundColor Yellow
    exit 1
}

# Run deployment
Start-Deployment
