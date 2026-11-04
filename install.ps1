param([string]$Browser)

& {
    $ErrorActionPreference = 'Stop'

    $browsers = 'chrome', 'chromium', 'brave', 'edge', 'vivaldi', 'opera', 'arc', 'helium'
    if (-not $Browser) {
        for ($i = 0; $i -lt $browsers.Count; $i++) {
            Write-Host "  $($i + 1)) $($browsers[$i])"
        }
        $choice = Read-Host "Which browser? [1-$($browsers.Count)]"
        if ($choice -match '^\d+$' -and [int]$choice -ge 1 -and [int]$choice -le $browsers.Count) {
            $Browser = $browsers[[int]$choice - 1]
        }
    }
    if ($browsers -notcontains $Browser) {
        Write-Host "Pass one of: $($browsers -join ', ')" -ForegroundColor Red
        return
    }

    $dir = Join-Path $env:LOCALAPPDATA 'Programs\chromium-profile-picker'
    $exe = Join-Path $dir 'chromium-profile-picker.exe'
    $url = "https://github.com/donnybeelo/chromium-profile-picker/releases/latest/download/chromium-profile-picker-$Browser-x86_64-pc-windows-msvc.exe"

    New-Item -ItemType Directory -Force -Path $dir | Out-Null

    Write-Host "Downloading chromium-profile-picker for $Browser..."
    try {
        Invoke-WebRequest -Uri $url -OutFile $exe -UseBasicParsing
    } catch [System.IO.IOException] {
        Write-Host 'chromium-profile-picker is running. Close it, then run this again to update.' -ForegroundColor Red
        return
    }

    $path = [Environment]::GetEnvironmentVariable('Path', 'User')
    if (-not $path) { $path = '' }
    if (($path -split ';') -notcontains $dir) {
        $parts = @($path.TrimEnd(';'), $dir) | Where-Object { $_ }
        [Environment]::SetEnvironmentVariable('Path', ($parts -join ';'), 'User')
        $env:Path = "$env:Path;$dir"
        Write-Host "Added $dir to your PATH"
    }

    & $exe --set-default
    Write-Host 'Installed. Set it as your default browser in the settings page that just opened.' -ForegroundColor Green
}
