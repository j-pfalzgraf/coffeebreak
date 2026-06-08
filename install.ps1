<#
.SYNOPSIS
    Windows one-line installer for coffeebreak (a Pomodoro CLI focus timer).

.DESCRIPTION
    Downloads the appropriate release asset from GitHub, verifies its SHA-256
    against the published SHA256SUMS file, then installs the `coffeebreak.exe`
    binary into the per-user programs directory and adds it to the user PATH.

    Intended to be run as:
        irm https://raw.githubusercontent.com/j-pfalzgraf/coffeebreak/main/install.ps1 | iex

.NOTES
    Environment overrides:
        $env:COFFEEBREAK_VERSION  - release tag to install (e.g. v1.0.0). Default: latest.

    Security:
        - HTTPS only.
        - The downloaded archive's SHA-256 is verified against SHA256SUMS
          BEFORE anything is extracted or installed; a mismatch aborts.
        - Downloads happen in a private temp directory that is always removed.
#>

# --- Strict execution -------------------------------------------------------
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

# Windows PowerShell 5.1 (the default shell for `irm ... | iex`) may negotiate
# TLS 1.0/1.1, which GitHub rejects. Force TLS 1.2+ so downloads succeed.
# Harmless on PowerShell 7+, which already uses the OS default.
try {
    [Net.ServicePointManager]::SecurityProtocol = `
        [Net.ServicePointManager]::SecurityProtocol -bor [Net.SecurityProtocolType]::Tls12
}
catch {
    # Some editions don't expose this; ignore and proceed.
}

# --- Constants --------------------------------------------------------------
# GitHub repo coordinates (deliberate placeholder owner/name — do not change).
$Owner = 'j-pfalzgraf'
$Repo  = 'coffeebreak'

# Install location: %LOCALAPPDATA%\Programs\coffeebreak\coffeebreak.exe
$InstallDir = Join-Path $env:LOCALAPPDATA 'Programs\coffeebreak'
$BinaryName = 'coffeebreak.exe'

# --- Helpers ----------------------------------------------------------------

function Write-Info {
    param([string]$Message)
    Write-Host $Message
}

function Fail {
    param([string]$Message)
    # Throwing under $ErrorActionPreference='Stop' aborts the whole script with
    # a clear, non-zero outcome.
    throw "coffeebreak install error: $Message"
}

# Detect the Rust target triple for this machine from the processor arch.
function Get-Target {
    $arch = $env:PROCESSOR_ARCHITECTURE
    switch ($arch) {
        'AMD64' { return 'x86_64-pc-windows-msvc' }
        'ARM64' { return 'aarch64-pc-windows-msvc' }
        default {
            Fail "unsupported architecture '$arch'. Supported: AMD64, ARM64."
        }
    }
}

# Download a URL to a file using Invoke-WebRequest (HTTPS only).
function Get-RemoteFile {
    param(
        [string]$Url,
        [string]$OutFile
    )
    if ($Url -notmatch '^https://') {
        Fail "refusing to download non-HTTPS URL: $Url"
    }
    try {
        # -UseBasicParsing keeps this working on hosts without the IE engine.
        Invoke-WebRequest -Uri $Url -OutFile $OutFile -UseBasicParsing
    }
    catch {
        Fail "failed to download $Url`: $($_.Exception.Message)"
    }
}

# Parse a SHA256SUMS file (sha256sum format: "<hex><two spaces><filename>")
# and return the lowercase hex digest recorded for the given asset filename.
function Get-ExpectedHash {
    param(
        [string]$SumsFile,
        [string]$AssetName
    )
    foreach ($line in Get-Content -LiteralPath $SumsFile) {
        if ([string]::IsNullOrWhiteSpace($line)) { continue }
        # Split on whitespace; sha256sum uses "<hash><two spaces><name>".
        # The filename may carry a leading "*" (binary mode marker) — strip it.
        $parts = $line -split '\s+', 2
        if ($parts.Count -ne 2) { continue }
        $hash = $parts[0].Trim().ToLowerInvariant()
        $name = $parts[1].Trim().TrimStart('*')
        if ($name -eq $AssetName) {
            return $hash
        }
    }
    Fail "could not find an entry for '$AssetName' in SHA256SUMS."
}

# --- Resolve target, version, URLs ------------------------------------------

$Target = Get-Target

# Asset filename convention (must stay byte-identical across installers):
#   coffeebreak-<TARGET>.zip
$AssetName = "coffeebreak-$Target.zip"
$SumsName  = 'SHA256SUMS'

# Version: default to the "latest" release; allow an explicit tag override.
$Version = $env:COFFEEBREAK_VERSION
if ([string]::IsNullOrWhiteSpace($Version)) {
    $Version  = 'latest'
    $BaseUrl  = "https://github.com/$Owner/$Repo/releases/latest/download"
}
else {
    $BaseUrl  = "https://github.com/$Owner/$Repo/releases/download/$Version"
}

$AssetUrl = "$BaseUrl/$AssetName"
$SumsUrl  = "$BaseUrl/$SumsName"

# --- Work in a private temp directory, cleaned up no matter what ------------

# Create a unique temp dir under the system temp path.
$TmpRoot = [System.IO.Path]::GetTempPath()
$TmpDir  = Join-Path $TmpRoot ("coffeebreak-install-" + [System.Guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $TmpDir -Force | Out-Null

try {
    $AssetPath = Join-Path $TmpDir $AssetName
    $SumsPath  = Join-Path $TmpDir $SumsName

    Write-Info "Downloading coffeebreak ($Target, version: $Version)..."
    Write-Info "  source: $AssetUrl"

    Get-RemoteFile -Url $AssetUrl -OutFile $AssetPath
    Get-RemoteFile -Url $SumsUrl  -OutFile $SumsPath

    # --- Verify checksum BEFORE extracting/installing -----------------------
    $expected = Get-ExpectedHash -SumsFile $SumsPath -AssetName $AssetName
    $actual   = (Get-FileHash -Algorithm SHA256 -LiteralPath $AssetPath).Hash.ToLowerInvariant()

    if ($actual -ne $expected) {
        Fail @"
checksum verification FAILED for $AssetName
  expected: $expected
  actual:   $actual
Aborting without installing.
"@
    }
    Write-Info "Checksum verified (SHA256)."

    # --- Announce before installing -----------------------------------------
    Write-Info ""
    Write-Info "Installing coffeebreak"
    Write-Info "  version: $Version"
    Write-Info "  source:  $AssetUrl"
    Write-Info "  target:  $InstallDir"
    Write-Info ""

    # --- Extract and place the binary ---------------------------------------
    # Ensure the install dir exists (idempotent).
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null

    # Expand into a staging subdir, then copy the binary out. This keeps the
    # logic robust and lets us overwrite an existing install cleanly.
    $ExtractDir = Join-Path $TmpDir 'extract'
    New-Item -ItemType Directory -Path $ExtractDir -Force | Out-Null
    Expand-Archive -LiteralPath $AssetPath -DestinationPath $ExtractDir -Force

    # The archive must contain coffeebreak.exe at its root.
    $SrcBinary = Join-Path $ExtractDir $BinaryName
    if (-not (Test-Path -LiteralPath $SrcBinary)) {
        Fail "archive did not contain '$BinaryName' at its root."
    }

    $DestBinary = Join-Path $InstallDir $BinaryName
    Copy-Item -LiteralPath $SrcBinary -Destination $DestBinary -Force

    # --- Ensure the install dir is on the USER PATH -------------------------
    $userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
    if ($null -eq $userPath) { $userPath = '' }

    # Compare path entries case-insensitively, ignoring empties.
    $onPath = $false
    foreach ($entry in ($userPath -split ';')) {
        if ([string]::IsNullOrWhiteSpace($entry)) { continue }
        if ($entry.TrimEnd('\') -ieq $InstallDir.TrimEnd('\')) {
            $onPath = $true
            break
        }
    }

    if (-not $onPath) {
        $newPath = if ([string]::IsNullOrWhiteSpace($userPath)) {
            $InstallDir
        }
        else {
            ($userPath.TrimEnd(';') + ';' + $InstallDir)
        }
        [Environment]::SetEnvironmentVariable('Path', $newPath, 'User')
        # Reflect it in the current session too, for immediate use. Guard against
        # a null/empty process PATH (calling .TrimEnd on $null would throw).
        $sessionPath = if ([string]::IsNullOrEmpty($env:Path)) { '' } else { $env:Path.TrimEnd(';') }
        $env:Path = if ($sessionPath) { $sessionPath + ';' + $InstallDir } else { $InstallDir }
        Write-Info "Added '$InstallDir' to your USER PATH."
        Write-Info "Restart your shell (or sign out/in) for the PATH change to take effect everywhere."
    }

    # --- Success ------------------------------------------------------------
    Write-Info ""
    Write-Info "coffeebreak installed successfully to:"
    Write-Info "  $DestBinary"
    Write-Info ""
    Write-Info "Get started:"
    Write-Info "  coffeebreak              # 25 min focus / 5 min break"
    Write-Info "  coffeebreak --stats      # show your stats"
    Write-Info "  coffeebreak self update  # update later"
}
finally {
    # Always clean up the private temp directory.
    if (Test-Path -LiteralPath $TmpDir) {
        Remove-Item -LiteralPath $TmpDir -Recurse -Force -ErrorAction SilentlyContinue
    }
}
