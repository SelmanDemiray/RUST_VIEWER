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

# PowerShell script to clean up build artifacts and temporary files

Write-Host "Cleaning up Rust project..." -ForegroundColor Green

# Clean Cargo build artifacts
if (Test-Path "target") {
    Write-Host "Removing target directory..." -ForegroundColor Yellow
    Remove-Item "target" -Recurse -Force
    Write-Host "Removed target directory" -ForegroundColor Green
}

# Clean Cargo lock file if needed (optional)
if ($args -contains "--clean-lock") {
    if (Test-Path "Cargo.lock") {
        Write-Host "Removing Cargo.lock..." -ForegroundColor Yellow
        Remove-Item "Cargo.lock" -Force
        Write-Host "Removed Cargo.lock" -ForegroundColor Green
    }
}

# Remove any backup files
$backupFiles = Get-ChildItem -Recurse -Name "*.rs.bak", "*.toml.bak", "*.orig"
if ($backupFiles.Count -gt 0) {
    Write-Host "Removing backup files..." -ForegroundColor Yellow
    foreach ($file in $backupFiles) {
        Remove-Item $file -Force
        Write-Host "Removed $file" -ForegroundColor Green
    }
}

# Remove any temporary editor files
$tempFiles = Get-ChildItem -Recurse -Name "*~", "*.tmp", "*.swp"
if ($tempFiles.Count -gt 0) {
    Write-Host "Removing temporary files..." -ForegroundColor Yellow
    foreach ($file in $tempFiles) {
        Remove-Item $file -Force
        Write-Host "Removed $file" -ForegroundColor Green
    }
}

Write-Host "Cleanup completed!" -ForegroundColor Green
Write-Host "You can now run: cargo build --release" -ForegroundColor Blue
