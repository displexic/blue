#!/usr/bin/env pwsh

# Stop executing script on any error
$ErrorActionPreference = 'Stop'
# Do not show download progress
$ProgressPreference = 'SilentlyContinue'

# Taken from https://stackoverflow.com/a/34559554/6537420
function New-TemporaryDirectory {
  $parent = [System.IO.Path]::GetTempPath()
  [string] $name = [System.Guid]::NewGuid()
  New-Item -ItemType Directory -Path (Join-Path $parent $name)
}

$platform = $null
$architecture = $null
$fileName = $null

# PowerShell versions before 6.* were only for Windows OS
if ($PSVersionTable.PSVersion.Major -eq 5) {
  $platform = 'win'
}

if ($PSVersionTable.PSVersion.Major -ge 6) {
  if ($PSVersionTable.Platform -eq 'Unix') {
    if ($PSVersionTable.OS -like 'Darwin*') {
      $platform = 'darwin'
    }

    if ($PSVersionTable.OS -like 'Linux*') {
      $platform = 'linux'
    }

    # PowerShell does not seem to have normal cmdlets for retrieving system information, so we use UNAME(1) for this.
    $arch = uname -m
    switch -Wildcard ($arch) {
      'x86_64' { $architecture = 'amd64'; Break }
      'amd64' { $architecture = 'amd64'; Break }
      'armv*' { $architecture = 'arm'; Break }
      'arm64' { $architecture = 'arm64'; Break }
      'aarch64' { $architecture = 'arm64'; Break }
    }

    # 'uname -m' in some cases mis-reports 32-bit OS as 64-bit, so double check
    if ([System.Environment]::Is64BitOperatingSystem -eq $false) {
      if ($architecture -eq 'amd64') {
        $architecture = 'i686'
      }

      if ($architecture -eq 'arm64') {
        $architecture = 'arm'
      }
    }

    $fileName = "blue"
  }

  if ($PSVersionTable.Platform -eq 'Win32NT') {
    $platform = 'win'
  }
}

if ($platform -eq 'win') {
  if ([System.Environment]::Is64BitOperatingSystem -eq $true) {
    $architecture = 'amd64'
  }

  if ([System.Environment]::Is64BitOperatingSystem -eq $false) {
    $architecture = 'i686'
  }

  $fileName = "blue.exe"
}

if ($null -eq $platform) {
  Write-Error "Platform could not be determined! Only Windows, Linux and macOS are supported."
}

switch ($architecture) {
  'amd64' { ; Break }
  'arm64' { ; Break }
  Default {
    Write-Error "Sorry! Blue currently only provides pre-built binaries for x86_64/arm64 architectures."
  }
}

# The name of the bin file to download
$file = "blue-$platform-$architecture"
if ($platform -eq 'win') {
  $file="$file.exe"
}

# Determine latest release tag
$releases = "https://api.github.com/repos/slekup/blue/releases"
$tag = (Invoke-WebRequest $releases | ConvertFrom-Json)[0].tag_name
$download = "https://github.com/slekup/blue/releases/download/$tag/$file"

Write-Host "Downloading Blue from latest GitHub release...`n" -ForegroundColor Green

$tempFileFolder = New-TemporaryDirectory
$tempFile = (Join-Path $tempFileFolder.FullName $fileName)

Invoke-WebRequest $download -OutFile $tempFile -UseBasicParsing
Invoke-WebRequest $archiveUrl -OutFile $tempFile

Write-Host "Running Blue setup...`n" -ForegroundColor Green

if ($platform -ne 'win') {
  chmod +x $tempFile
}

Start-Process -FilePath $tempFile -ArgumentList "setup" -NoNewWindow -Wait -ErrorAction Continue

Remove-Item $tempFile
Remove-Item $tempFileFolder -Recurse