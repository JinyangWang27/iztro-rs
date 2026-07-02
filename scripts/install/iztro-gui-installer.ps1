#Requires -Version 5.1
$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$REPO = "JinyangWang27/iztro-rs"
$BASE_URL = if ($env:IZTRO_GUI_RELEASE_URL) { $env:IZTRO_GUI_RELEASE_URL } else { "https://github.com/$REPO/releases/latest/download" }

if ($BASE_URL -notmatch '^https://') {
    Write-Error "error: release URL must use HTTPS: $BASE_URL"
    exit 1
}

$INSTALL_DIR = if ($env:IZTRO_GUI_INSTALL_DIR) {
    $env:IZTRO_GUI_INSTALL_DIR
} else {
    if (-not $env:LOCALAPPDATA) {
        Write-Error "error: LOCALAPPDATA is not set; set IZTRO_GUI_INSTALL_DIR to choose an install directory"
        exit 1
    }
    Join-Path $env:LOCALAPPDATA "Programs\iztro-gui"
}

$ARTIFACT     = "iztro-gui-x86_64-pc-windows-msvc.zip"
$ARTIFACT_DIR = "iztro-gui-x86_64-pc-windows-msvc"
$CHECKSUM     = "$ARTIFACT.sha256"

$TMP_DIR = Join-Path ([System.IO.Path]::GetTempPath()) ([System.IO.Path]::GetRandomFileName())

try {
    New-Item -ItemType Directory -Force -Path $TMP_DIR | Out-Null

    $archive_path  = Join-Path $TMP_DIR $ARTIFACT
    $checksum_path = Join-Path $TMP_DIR $CHECKSUM

    Write-Host "Downloading $ARTIFACT"
    Invoke-WebRequest -Uri "$BASE_URL/$ARTIFACT"         -OutFile $archive_path  -UseBasicParsing
    Invoke-WebRequest -Uri "$BASE_URL/$CHECKSUM"         -OutFile $checksum_path -UseBasicParsing

    $expected_raw = (Get-Content $checksum_path -Raw).Trim() -split '\s+' | Select-Object -First 1
    $expected = $expected_raw.ToLowerInvariant()
    $actual   = (Get-FileHash -Algorithm SHA256 $archive_path).Hash.ToLowerInvariant()

    if ($expected -ne $actual) {
        Write-Error "error: checksum verification failed for $ARTIFACT`nexpected: $expected`nactual:   $actual"
        exit 1
    }

    Expand-Archive -Path $archive_path -DestinationPath $TMP_DIR -Force

    $extracted_exe = Join-Path $TMP_DIR "$ARTIFACT_DIR\iztro-gui.exe"
    if (-not (Test-Path $extracted_exe)) {
        Write-Error "error: archive did not contain $ARTIFACT_DIR\iztro-gui.exe"
        exit 1
    }

    if (-not (Test-Path $INSTALL_DIR)) {
        New-Item -ItemType Directory -Force -Path $INSTALL_DIR | Out-Null
    }

    $target     = Join-Path $INSTALL_DIR "iztro-gui.exe"
    $tmp_target = Join-Path $INSTALL_DIR ".iztro-gui.tmp.$PID.exe"

    if (Test-Path $target) {
        Write-Host "Replacing existing $target"
    }

    Copy-Item $extracted_exe $tmp_target -Force
    Move-Item  $tmp_target   $target     -Force

    Write-Host "Installed iztro-gui to $target"
} finally {
    if (Test-Path $TMP_DIR) {
        Remove-Item -Recurse -Force $TMP_DIR -ErrorAction SilentlyContinue
    }
}
