<#
.SYNOPSIS
Runs clang-format over the project's C/C++ headers and sources, excluding vendored files.

.PARAMETER Check
Report unformatted files instead of rewriting them. Exits with 1 if any need formatting.

.EXAMPLE
./scripts/format.ps1
./scripts/format.ps1 -Check
#>
param([switch]$Check)

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
Push-Location $root
try {
    if (-not (Get-Command clang-format -ErrorAction SilentlyContinue)) {
        throw 'clang-format not found on PATH.'
    }

    # Tracked and untracked files, minus gitignored ones (target/, external_assets/, ...).
    # The nifly submodule isn't listed, but exclude vendor/ explicitly in case that changes.
    $files = git ls-files --cached --others --exclude-standard -- '*.h' '*.hpp' '*.c' '*.cc' '*.cpp' |
        Where-Object { $_ -notmatch '(^|/)vendor/' } |
        Sort-Object -Unique

    if (-not $files) {
        Write-Host 'No files to format.'
        return
    }

    if ($Check) {
        clang-format --dry-run --Werror @files
        if ($LASTEXITCODE -ne 0) {
            Write-Host 'Some files need formatting. Run ./scripts/format.ps1 to fix them.'
            exit 1
        }
        Write-Host "$($files.Count) files are formatted."
    }
    else {
        clang-format -i @files
        if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
        Write-Host "Formatted $($files.Count) files:"
        $files | ForEach-Object { Write-Host "  $_" }
    }
}
finally {
    Pop-Location
}
