# Test PowerShell syntax
function Test-Syntax {
    $installDir = "C:\Test"
    $userPath = "C:\Windows"
    $systemPath = "C:\Windows\System32"
    
    if ($true) {
        # User-level PATH
        if ($userPath -notlike "*$installDir*") {
            $newUserPath = $userPath + ";" + $installDir
            Write-Host "Would set user PATH to: $newUserPath"
        }
    } else {
        # System-level PATH
        if ($systemPath -notlike "*$installDir*") {
            $newSystemPath = $systemPath + ";" + $installDir
            Write-Host "Would set system PATH to: $newSystemPath"
        }
    }
}

Test-Syntax
