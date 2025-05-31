param(
    [switch]$Auto = $false,
    [string]$DefaultDialogAction = "3",     # 1=rename simple_dialog.rs, 2=update main.rs, 3=skip
    [switch]$AutoDeleteUnused = $false,     # Whether to auto-delete unused files in auto mode
    [switch]$AutoDeleteTemp = $true        # Whether to auto-delete temporary files in auto mode
)

Write-Host "Rust Viewer Project Cleanup Script" -ForegroundColor Cyan
Write-Host "=================================" -ForegroundColor Cyan

if ($Auto) {
    Write-Host "Running in automatic mode" -ForegroundColor Green
    Write-Host "  - Default dialog action: $DefaultDialogAction" -ForegroundColor Green
    Write-Host "  - Auto-delete unused files: $AutoDeleteUnused" -ForegroundColor Green
    Write-Host "  - Auto-delete temp files: $AutoDeleteTemp" -ForegroundColor Green
}

# Go to project root directory
Set-Location "d:\RUST_VIEWER"

# Check for module name mismatch between simple_dialog.rs and dialog.rs
$dialogExists = Test-Path "src\dialog.rs"
$simpleDialogExists = Test-Path "src\simple_dialog.rs"

if ($dialogExists -and $simpleDialogExists) {
    Write-Host "Found both dialog.rs and simple_dialog.rs" -ForegroundColor Yellow
    Write-Host "Checking main.rs to see which one is actually used..." -ForegroundColor Yellow
    
    $mainContent = Get-Content "src\main.rs" -Raw
    if ($mainContent -match "mod dialog;" -and -not ($mainContent -match "mod simple_dialog;")) {
        Write-Host "main.rs imports 'dialog' but you have 'simple_dialog.rs'" -ForegroundColor Red
        
        if (-not $Auto) {
            Write-Host "Would you like to: "
            Write-Host "1. Rename simple_dialog.rs to dialog.rs (replacing the existing dialog.rs)"
            Write-Host "2. Update main.rs to use simple_dialog instead"
            Write-Host "3. Skip this issue"
            
            $choice = Read-Host "Enter your choice (1-3)"
        } else {
            $choice = $DefaultDialogAction
            Write-Host "Auto-selecting option $choice for dialog module conflict" -ForegroundColor Yellow
        }
        
        switch ($choice) {
            "1" {
                Write-Host "Replacing dialog.rs with simple_dialog.rs..." -ForegroundColor Yellow
                Remove-Item "src\dialog.rs" -Force
                Rename-Item "src\simple_dialog.rs" "dialog.rs"
                Write-Host "Done." -ForegroundColor Green
            }
            "2" {
                Write-Host "Updating main.rs to reference simple_dialog..." -ForegroundColor Yellow
                $newContent = $mainContent -replace "mod dialog;", "mod simple_dialog;"
                Set-Content "src\main.rs" $newContent
                Write-Host "Done." -ForegroundColor Green
            }
            default {
                Write-Host "Skipping this issue." -ForegroundColor Yellow
            }
        }
    }
}

# Check for unused files by examining imports in main.rs
$mainContent = Get-Content "src\main.rs" -Raw
$importedModules = [regex]::Matches($mainContent, "mod ([a-zA-Z_0-9]+);") | 
                  ForEach-Object { $_.Groups[1].Value }

# Track changes for summary report
$unusedFilesFound = @()
$unusedFilesDeleted = @()
$tempFilesFound = @()
$tempFilesDeleted = @()

# Check for files in src/ that aren't imported
$allSrcFiles = Get-ChildItem "src\*.rs" | Where-Object { $_.Name -ne "main.rs" }
foreach ($file in $allSrcFiles) {
    $moduleName = [System.IO.Path]::GetFileNameWithoutExtension($file.Name)
    if ($importedModules -notcontains $moduleName) {
        $unusedFilesFound += $file.Name
        Write-Host "Found potentially unused file: $($file.Name)" -ForegroundColor Yellow
        
        $delete = "no"
        if (-not $Auto) {
            $delete = Read-Host "Delete this file? (yes/no)"
        } elseif ($AutoDeleteUnused) {
            $delete = "yes"
            Write-Host "Auto-deleting unused file: $($file.Name)" -ForegroundColor Yellow
        } else {
            Write-Host "Skipping unused file in auto mode (AutoDeleteUnused is $AutoDeleteUnused)" -ForegroundColor Yellow
        }
        
        if ($delete -eq "yes") {
            Remove-Item $file.FullName -Force
            $unusedFilesDeleted += $file.Name
            Write-Host "Deleted $($file.Name)" -ForegroundColor Green
        } else {
            Write-Host "Keeping $($file.Name)" -ForegroundColor Yellow
        }
    }
}

# Check for temp/backup files
$tempPatterns = @("*.bak", "*.tmp", "*~", "*.swp", "*.old")
foreach ($pattern in $tempPatterns) {
    $tempFiles = Get-ChildItem -Path "." -Recurse -File -Filter $pattern
    foreach ($file in $tempFiles) {
        $tempFilesFound += $file.FullName
        Write-Host "Found temporary file: $($file.FullName)" -ForegroundColor Yellow
        
        $delete = "no"
        if (-not $Auto) {
            $delete = Read-Host "Delete this temporary file? (yes/no)"
        } elseif ($AutoDeleteTemp) {
            $delete = "yes"
            Write-Host "Auto-deleting temporary file: $($file.FullName)" -ForegroundColor Yellow
        } else {
            Write-Host "Skipping temporary file in auto mode (AutoDeleteTemp is $AutoDeleteTemp)" -ForegroundColor Yellow
        }
        
        if ($delete -eq "yes") {
            Remove-Item $file.FullName -Force
            $tempFilesDeleted += $file.FullName
            Write-Host "Deleted $($file.FullName)" -ForegroundColor Green
        } else {
            Write-Host "Keeping $($file.FullName)" -ForegroundColor Yellow
        }
    }
}

Write-Host "Cleanup complete!" -ForegroundColor Green

# Remove any conflicting editor module files
if (Test-Path "src\editor\mod.rs") {
    Remove-Item "src\editor\mod.rs" -Force
    Write-Host "Removed conflicting src\editor\mod.rs"
}

if (Test-Path "src\editor" -PathType Container) {
    $editorDir = Get-ChildItem "src\editor" -ErrorAction SilentlyContinue
    if ($editorDir.Count -eq 0) {
        Remove-Item "src\editor" -Force
        Write-Host "Removed empty src\editor directory"
    }
}

# Remove the conflicting editor.rs file
if (Test-Path "src\editor.rs") {
    Remove-Item "src\editor.rs" -Force
    Write-Host "Removed conflicting src\editor.rs file"
} else {
    Write-Host "No conflicting editor.rs file found"
}

# Clean up any build artifacts that might be causing issues
if (Test-Path "target") {
    Write-Host "Cleaning build artifacts..."
    cargo clean
} else {
    Write-Host "No target directory found"
}

Write-Host "Cleanup completed!" -ForegroundColor Green

# Print summary if in Auto mode
if ($Auto) {
    Write-Host "`nCleanup Summary:" -ForegroundColor Cyan
    Write-Host "------------------" -ForegroundColor Cyan
    
    Write-Host "Unused files found: $($unusedFilesFound.Count)" -ForegroundColor White
    if ($unusedFilesFound.Count -gt 0) {
        foreach ($file in $unusedFilesFound) {
            $status = if ($unusedFilesDeleted -contains $file) { "DELETED" } else { "KEPT" }
            Write-Host "  - $file ($status)" -ForegroundColor $(if ($status -eq "DELETED") { "Yellow" } else { "White" })
        }
    }
    
    Write-Host "Temporary files found: $($tempFilesFound.Count)" -ForegroundColor White
    if ($tempFilesFound.Count -gt 0) {
        foreach ($file in $tempFilesFound) {
            $status = if ($tempFilesDeleted -contains $file) { "DELETED" } else { "KEPT" }
            Write-Host "  - $file ($status)" -ForegroundColor $(if ($status -eq "DELETED") { "Yellow" } else { "White" })
        }
    }
}

Write-Host "You may still need to update imports if you deleted any files." -ForegroundColor Cyan

# Clean up build artifacts and reset project

Write-Host "Cleaning up Rust project..." -ForegroundColor Green

# Remove target directory
if (Test-Path "target") {
    Write-Host "Removing target directory..." -ForegroundColor Yellow
    Remove-Item "target" -Recurse -Force
}

# Remove Cargo.lock (it will be regenerated)
if (Test-Path "Cargo.lock") {
    Write-Host "Removing Cargo.lock..." -ForegroundColor Yellow
    Remove-Item "Cargo.lock" -Force
}

# Remove any backup files
Get-ChildItem -Recurse -Filter "*.rs.bk" | Remove-Item -Force
Get-ChildItem -Recurse -Filter "*~" | Remove-Item -Force

Write-Host "Cleanup complete!" -ForegroundColor Green
Write-Host "Run 'cargo build' to rebuild the project." -ForegroundColor Cyan

# Rust Code Visualizer - Cleanup Script
# This script cleans up build artifacts and temporary files

Write-Host "Rust Code Visualizer - Cleanup Script" -ForegroundColor Green
Write-Host "=====================================" -ForegroundColor Green

# Function to remove directory safely
function Remove-DirectorySafely {
    param([string]$Path, [string]$Description)
    
    if (Test-Path $Path) {
        try {
            Remove-Item -Recurse -Force $Path
            Write-Host "✓ Removed $Description" -ForegroundColor Green
        }
        catch {
            Write-Host "✗ Failed to remove $Description : $($_.Exception.Message)" -ForegroundColor Red
        }
    } else {
        Write-Host "- $Description not found (already clean)" -ForegroundColor Gray
    }
}

# Function to remove file safely
function Remove-FileSafely {
    param([string]$Path, [string]$Description)
    
    if (Test-Path $Path) {
        try {
            Remove-Item -Force $Path
            Write-Host "✓ Removed $Description" -ForegroundColor Green
        }
        catch {
            Write-Host "✗ Failed to remove $Description : $($_.Exception.Message)" -ForegroundColor Red
        }
    } else {
        Write-Host "- $Description not found (already clean)" -ForegroundColor Gray
    }
}

# Function to get directory size
function Get-DirectorySize {
    param([string]$Path)
    
    if (Test-Path $Path) {
        $size = (Get-ChildItem -Recurse $Path | Measure-Object -Property Length -Sum).Sum
        return [math]::Round($size / 1MB, 2)
    }
    return 0
}

# Function to clean build artifacts
function Clear-BuildArtifacts {
    Write-Host "Cleaning build artifacts..." -ForegroundColor Yellow
    
    $targetSize = Get-DirectorySize "target"
    if ($targetSize -gt 0) {
        Write-Host "Target directory size: $targetSize MB" -ForegroundColor Cyan
    }
    
    Remove-DirectorySafely "target" "target directory (build artifacts)"
    Remove-FileSafely "Cargo.lock" "Cargo.lock (dependency lock file)"
}

# Function to clean temporary files
function Clear-TemporaryFiles {
    Write-Host "Cleaning temporary files..." -ForegroundColor Yellow
    
    # Remove common temporary file patterns
    $tempPatterns = @(
        "*.tmp",
        "*.temp",
        "*~",
        "*.bak",
        "*.orig",
        ".DS_Store",
        "Thumbs.db"
    )
    
    foreach ($pattern in $tempPatterns) {
        $files = Get-ChildItem -Recurse -Filter $pattern -File -ErrorAction SilentlyContinue
        foreach ($file in $files) {
            Remove-FileSafely $file.FullName "temporary file ($($file.Name))"
        }
    }
}

# Function to clean IDE files
function Clear-IDEFiles {
    Write-Host "Cleaning IDE/editor files..." -ForegroundColor Yellow
    
    # Visual Studio Code
    Remove-DirectorySafely ".vscode" ".vscode directory"
    
    # IntelliJ/CLion
    Remove-DirectorySafely ".idea" ".idea directory"
    
    # Vim
    $vimFiles = Get-ChildItem -Recurse -Filter "*.swp" -File -ErrorAction SilentlyContinue
    foreach ($file in $vimFiles) {
        Remove-FileSafely $file.FullName "vim swap file ($($file.Name))"
    }
    
    # Emacs
    $emacsFiles = Get-ChildItem -Recurse -Filter "*~" -File -ErrorAction SilentlyContinue
    foreach ($file in $emacsFiles) {
        Remove-FileSafely $file.FullName "emacs backup file ($($file.Name))"
    }
}

# Function to clean logs
function Clear-LogFiles {
    Write-Host "Cleaning log files..." -ForegroundColor Yellow
    
    $logPatterns = @("*.log", "*.out", "*.err")
    
    foreach ($pattern in $logPatterns) {
        $files = Get-ChildItem -Recurse -Filter $pattern -File -ErrorAction SilentlyContinue
        foreach ($file in $files) {
            Remove-FileSafely $file.FullName "log file ($($file.Name))"
        }
    }
}

# Function to show disk space saved
function Show-SpaceSaved {
    Write-Host "Cleanup complete!" -ForegroundColor Green
    Write-Host "You may want to run 'cargo clean' if you need to ensure a completely fresh build." -ForegroundColor Cyan
}

# Function to clean everything
function Clear-All {
    Clear-BuildArtifacts
    Clear-TemporaryFiles
    Clear-IDEFiles
    Clear-LogFiles
    Show-SpaceSaved
}

# Function to clean only build artifacts
function Clear-BuildOnly {
    Clear-BuildArtifacts
    Show-SpaceSaved
}

# Function to clean only temporary files
function Clear-TempOnly {
    Clear-TemporaryFiles
    Show-SpaceSaved
}

# Main execution
function Main {
    param([string]$Action = "all")
    
    switch ($Action.ToLower()) {
        "all" {
            Clear-All
        }
        "build" {
            Clear-BuildOnly
        }
        "temp" {
            Clear-TempOnly
        }
        "ide" {
            Clear-IDEFiles
            Show-SpaceSaved
        }
        "logs" {
            Clear-LogFiles
            Show-SpaceSaved
        }
        default {
            Write-Host "Usage: .\cleanup.ps1 [all|build|temp|ide|logs]" -ForegroundColor Cyan
            Write-Host "  all   - Clean everything (default)"
            Write-Host "  build - Clean only build artifacts"
            Write-Host "  temp  - Clean only temporary files"
            Write-Host "  ide   - Clean only IDE/editor files"
            Write-Host "  logs  - Clean only log files"
        }
    }
}

# Check if we're in the right directory
if (-not (Test-Path "Cargo.toml")) {
    Write-Host "Error: Cargo.toml not found. Please run this script from the project root directory." -ForegroundColor Red
    exit 1
}

# Run with the provided parameter or default to "all"
Main -Action $args[0]

# Cleanup script for Rust Code Visualizer
# Removes build artifacts and temporary files

Write-Host "Cleaning up Rust Code Visualizer build artifacts..." -ForegroundColor Green

# Remove Cargo build artifacts
if (Test-Path "target") {
    Write-Host "Removing target directory..." -ForegroundColor Yellow
    Remove-Item -Recurse -Force "target"
}

# Remove Cargo lock file (optional - uncomment if needed)
# if (Test-Path "Cargo.lock") {
#     Write-Host "Removing Cargo.lock..." -ForegroundColor Yellow
#     Remove-Item -Force "Cargo.lock"
# }

# Remove any temporary files
$tempFiles = @("*.tmp", "*.bak", "*~")
foreach ($pattern in $tempFiles) {
    $files = Get-ChildItem -Recurse -Force -Name $pattern -ErrorAction SilentlyContinue
    if ($files) {
        Write-Host "Removing temporary files matching $pattern..." -ForegroundColor Yellow
        $files | ForEach-Object { Remove-Item -Force $_ }
    }
}

# Remove IDE files (optional)
$ideFiles = @(".vscode", ".idea", "*.swp", "*.swo")
foreach ($pattern in $ideFiles) {
    if (Test-Path $pattern) {
        Write-Host "Removing IDE files: $pattern..." -ForegroundColor Yellow
        Remove-Item -Recurse -Force $pattern
    }
}

Write-Host "Cleanup completed!" -ForegroundColor Green
Write-Host "You can now run 'cargo build --release' for a fresh build." -ForegroundColor Cyan

# Cleanup script for Rust Code Visualizer project

Write-Host "Rust Code Visualizer - Cleanup Script" -ForegroundColor Green
Write-Host "====================================" -ForegroundColor Green

# Function to safely remove directory
function Remove-SafeDirectory {
    param([string]$Path, [string]$Description)
    
    if (Test-Path $Path) {
        Write-Host "Removing $Description..." -ForegroundColor Yellow
        try {
            # Try normal removal first
            Remove-Item $Path -Recurse -Force -ErrorAction Stop
            Write-Host "✓ $Description removed successfully" -ForegroundColor Green
        } catch {
            Write-Warning "Standard removal failed, trying alternative method..."
            try {
                # Alternative method using robocopy to clear directory
                $tempEmpty = New-TemporaryFile | Split-Path
                $emptyDir = Join-Path $tempEmpty "empty"
                New-Item -ItemType Directory -Path $emptyDir -Force | Out-Null
                robocopy $emptyDir $Path /MIR /NFL /NDL /NJH /NJS /NC /NS /NP
                Remove-Item $Path -Recurse -Force
                Remove-Item $emptyDir -Force
                Write-Host "✓ $Description removed successfully (alternative method)" -ForegroundColor Green
            } catch {
                Write-Warning "Could not remove $Description`: $_"
            }
        }
    } else {
        Write-Host "✓ $Description not found (already clean)" -ForegroundColor Gray
    }
}

# Clean build artifacts
Remove-SafeDirectory "target" "Build target directory"

# Clean Cargo cache (local)
Remove-SafeDirectory "Cargo.lock" "Cargo lock file"

# Clean temporary files
$tempFiles = @("*.tmp", "*.temp", "*.bak", "*~")
foreach ($pattern in $tempFiles) {
    $files = Get-ChildItem -Path "." -Filter $pattern -Recurse -File
    if ($files.Count -gt 0) {
        Write-Host "Removing temporary files ($pattern)..." -ForegroundColor Yellow
        $files | Remove-Item -Force
        Write-Host "✓ Removed $($files.Count) temporary files" -ForegroundColor Green
    }
}

# Clean IDE/editor files
$idePatterns = @(".vscode", ".idea", "*.swp", "*.swo")
foreach ($pattern in $idePatterns) {
    if ($pattern.StartsWith(".")) {
        Remove-SafeDirectory $pattern "IDE directory ($pattern)"
    } else {
        $files = Get-ChildItem -Path "." -Filter $pattern -Recurse -File
        if ($files.Count -gt 0) {
            Write-Host "Removing IDE files ($pattern)..." -ForegroundColor Yellow
            $files | Remove-Item -Force
            Write-Host "✓ Removed $($files.Count) IDE files" -ForegroundColor Green
        }
    }
}

# Clean Windows-specific files
if ($IsWindows -or $env:OS -eq "Windows_NT") {
    $windowsFiles = @("Thumbs.db", "desktop.ini", "*.lnk")
    foreach ($pattern in $windowsFiles) {
        $files = Get-ChildItem -Path "." -Filter $pattern -Recurse -File -Hidden -ErrorAction SilentlyContinue
        if ($files.Count -gt 0) {
            Write-Host "Removing Windows files ($pattern)..." -ForegroundColor Yellow
            $files | Remove-Item -Force
            Write-Host "✓ Removed $($files.Count) Windows files" -ForegroundColor Green
        }
    }
}

# Report final state
Write-Host "`nCleanup Summary:" -ForegroundColor Green
Write-Host "===============" -ForegroundColor Green

$currentSize = Get-ChildItem -Path "." -Recurse -File | Measure-Object -Property Length -Sum
Write-Host "Project size: $([math]::Round($currentSize.Sum / 1MB, 2)) MB" -ForegroundColor Cyan
Write-Host "File count: $($currentSize.Count)" -ForegroundColor Cyan

Write-Host "`nCleanup completed successfully!" -ForegroundColor Green
Write-Host "You can now run: cargo build --release" -ForegroundColor Yellow
