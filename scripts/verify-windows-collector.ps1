[CmdletBinding()]
param(
  [string]$ExePath = "target\release\collector-windows.exe",
  [string]$ReportDir = "dist",
  [switch]$RequireSignature
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Write-Section {
  param([string]$Name)
  Write-Host ""
  Write-Host "== $Name =="
}

function Find-WindowsSdkTool {
  param([Parameter(Mandatory = $true)][string]$ToolName)

  $command = Get-Command $ToolName -ErrorAction SilentlyContinue
  if ($null -ne $command) {
    return $command.Source
  }

  $kitsRoot = Join-Path ${env:ProgramFiles(x86)} "Windows Kits\10\bin"
  if (-not (Test-Path -LiteralPath $kitsRoot)) {
    return $null
  }

  $matches = @(Get-ChildItem -LiteralPath $kitsRoot -Recurse -Filter $ToolName -ErrorAction SilentlyContinue |
    Where-Object { $_.FullName -match "\\x64\\$([regex]::Escape($ToolName))$" } |
    Sort-Object FullName -Descending)

  if ($matches.Count -gt 0) {
    return $matches[0].FullName
  }

  return $null
}

function Find-DumpBin {
  $command = Get-Command "dumpbin.exe" -ErrorAction SilentlyContinue
  if ($null -ne $command) {
    return $command.Source
  }

  $vswhere = Join-Path ${env:ProgramFiles(x86)} "Microsoft Visual Studio\Installer\vswhere.exe"
  if (-not (Test-Path -LiteralPath $vswhere)) {
    return $null
  }

  $installPath = & $vswhere -latest -products "*" -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
  if ($LASTEXITCODE -ne 0 -or [string]::IsNullOrWhiteSpace($installPath)) {
    return $null
  }

  $toolsRoot = Join-Path $installPath "VC\Tools\MSVC"
  if (-not (Test-Path -LiteralPath $toolsRoot)) {
    return $null
  }

  $matches = @(Get-ChildItem -LiteralPath $toolsRoot -Recurse -Filter "dumpbin.exe" -ErrorAction SilentlyContinue |
    Where-Object { $_.FullName -match "\\bin\\Hostx64\\x64\\dumpbin\.exe$" } |
    Sort-Object FullName -Descending)

  if ($matches.Count -gt 0) {
    return $matches[0].FullName
  }

  return $null
}

function Invoke-ExeSmokeTest {
  param(
    [Parameter(Mandatory = $true)][string]$FilePath,
    [int]$TimeoutSeconds = 15
  )

  $processInfo = [System.Diagnostics.ProcessStartInfo]::new()
  $processInfo.FileName = $FilePath
  $processInfo.Arguments = "--version"
  $processInfo.UseShellExecute = $false
  $processInfo.RedirectStandardOutput = $true
  $processInfo.RedirectStandardError = $true
  $processInfo.CreateNoWindow = $true

  $process = [System.Diagnostics.Process]::new()
  $process.StartInfo = $processInfo

  [void]$process.Start()
  if (-not $process.WaitForExit($TimeoutSeconds * 1000)) {
    try {
      $process.Kill($true)
    } catch {
      $process.Kill()
    }
    throw "Executable smoke test timed out after $TimeoutSeconds seconds."
  }

  $stdout = $process.StandardOutput.ReadToEnd()
  $stderr = $process.StandardError.ReadToEnd()

  if ($process.ExitCode -ne 0) {
    throw "Executable smoke test failed with exit code $($process.ExitCode). stdout: $stdout stderr: $stderr"
  }
}

if (-not $IsWindows) {
  throw "This verifier must run on Windows because it uses Windows SDK and PE inspection tools."
}

$os = Get-CimInstance -ClassName Win32_OperatingSystem
$osVersion = [version]$os.Version
if ($osVersion.Major -lt 10) {
  throw "Unsupported Windows version for verification: $($os.Caption) $($os.Version). Windows 10 or newer is required."
}

$resolvedExe = Resolve-Path -LiteralPath $ExePath -ErrorAction Stop
$exe = $resolvedExe.Path
$exeItem = Get-Item -LiteralPath $exe

New-Item -ItemType Directory -Force -Path $ReportDir | Out-Null
$resolvedReportDir = Resolve-Path -LiteralPath $ReportDir
$reportDirPath = $resolvedReportDir.Path

Write-Section "Executable"
if ($exeItem.Length -le 0) {
  throw "Executable is empty: $exe"
}
Write-Host "Path: $exe"
Write-Host "Size: $($exeItem.Length) bytes"

Write-Section "Windows host"
Write-Host "Caption: $($os.Caption)"
Write-Host "Version: $($os.Version)"
Write-Host "Build: $($os.BuildNumber)"

Write-Section "Loader smoke test"
Invoke-ExeSmokeTest -FilePath $exe
Write-Host "collector-windows.exe --version exited successfully."

Write-Section "Hash"
$hash = Get-FileHash -LiteralPath $exe -Algorithm SHA256
$hashLine = "$($hash.Hash)  $($exeItem.Name)"
$exeHashPath = Join-Path $reportDirPath "collector-windows.exe.sha256"
$hashLine | Set-Content -LiteralPath $exeHashPath
Write-Host $hashLine

Write-Section "Authenticode"
$signature = Get-AuthenticodeSignature -FilePath $exe
Write-Host "Status: $($signature.Status)"
if ($signature.Status -eq "Valid") {
  Write-Host "Signer: $($signature.SignerCertificate.Subject)"
} elseif ($signature.Status -eq "NotSigned") {
  if ($RequireSignature) {
    throw "Executable is not signed. Add signing or run without -RequireSignature."
  }
  Write-Warning "Executable is not signed. This is allowed for now, but releases should eventually be Authenticode signed."
} else {
  throw "Executable signature status is $($signature.Status): $($signature.StatusMessage)"
}

Write-Section "Manifest"
$mt = Find-WindowsSdkTool "mt.exe"
if ([string]::IsNullOrWhiteSpace($mt)) {
  throw "mt.exe was not found. Install the Windows SDK."
}

$manifestPath = Join-Path $reportDirPath "collector-windows.embedded.manifest"
& $mt "-nologo" "-inputresource:$exe;#1" "-out:$manifestPath"
if ($LASTEXITCODE -ne 0) {
  throw "Failed to extract embedded manifest with mt.exe."
}

$manifestText = Get-Content -LiteralPath $manifestPath -Raw
if ($manifestText -notmatch "Microsoft\.Windows\.Common-Controls") {
  throw "Embedded manifest does not declare Microsoft.Windows.Common-Controls."
}
if ($manifestText -notmatch 'version="6\.0\.0\.0"') {
  throw "Embedded manifest does not request Common Controls v6."
}
if ($manifestText -notmatch "8e0f7a12-bfb3-4fe8-b9a5-48fd50a15a9a") {
  throw "Embedded manifest does not declare Windows 10/11 compatibility."
}
Write-Host "Common Controls v6 manifest dependency found."
Write-Host "Windows 10/11 compatibility declaration found."

Write-Section "Dependencies"
$dumpbin = Find-DumpBin
if ([string]::IsNullOrWhiteSpace($dumpbin)) {
  throw "dumpbin.exe was not found. Install Visual Studio Build Tools with the C++ workload."
}

$dependenciesPath = Join-Path $reportDirPath "collector-windows-dependencies.txt"
$dependentsOutput = & $dumpbin /dependents $exe 2>&1
$dependentsOutput | Set-Content -LiteralPath $dependenciesPath
if ($LASTEXITCODE -ne 0) {
  throw "dumpbin /dependents failed."
}

$dependencies = @($dependentsOutput |
  Where-Object { $_ -match '^\s+[A-Za-z0-9_.-]+\.dll\s*$' } |
  ForEach-Object { $_.Trim() } |
  Sort-Object -Unique)

if ($dependencies -notcontains "COMCTL32.dll") {
  throw "COMCTL32.dll was not listed as a dependency; the native UI dependency check may be invalid."
}
Write-Host "COMCTL32.dll dependency found."

$importsPath = Join-Path $reportDirPath "collector-windows-imports.txt"
$importsOutput = & $dumpbin /imports $exe 2>&1
$importsOutput | Set-Content -LiteralPath $importsPath
if ($LASTEXITCODE -ne 0) {
  throw "dumpbin /imports failed."
}

$subclassImport = if ($importsOutput -match "GetWindowSubclass") { "present" } else { "not listed" }
Write-Host "GetWindowSubclass import: $subclassImport"

Write-Section "App-local DLLs"
$appLocalDlls = @(Get-ChildItem -LiteralPath $exeItem.DirectoryName -Filter "*.dll" -File -ErrorAction SilentlyContinue)
if ($appLocalDlls.Count -gt 0) {
  Write-Warning "App-local DLLs can shadow system DLLs. Review these files:"
  $appLocalDlls | ForEach-Object { Write-Warning "  $($_.FullName)" }
} else {
  Write-Host "No app-local DLLs found next to the executable."
}

$reportPath = Join-Path $reportDirPath "collector-windows-integrity.txt"
$report = @(
  "Race Agent Windows collector integrity report"
  "Generated: $(Get-Date -Format o)"
  "Executable: $exe"
  "VerifierWindowsCaption: $($os.Caption)"
  "VerifierWindowsVersion: $($os.Version)"
  "VerifierWindowsBuild: $($os.BuildNumber)"
  "SizeBytes: $($exeItem.Length)"
  "SHA256: $($hash.Hash)"
  "AuthenticodeStatus: $($signature.Status)"
  "CommonControlsV6Manifest: present"
  "Windows10CompatibilityManifest: present"
  "COMCTL32Dependency: present"
  "GetWindowSubclassImport: $subclassImport"
  "AppLocalDllCount: $($appLocalDlls.Count)"
  ""
  "Dependencies:"
) + ($dependencies | ForEach-Object { "  $_" })

$report | Set-Content -LiteralPath $reportPath
Write-Host "Report written to $reportPath"
