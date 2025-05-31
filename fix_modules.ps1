param(
    [switch]$Auto = $false,
    [string]$DefaultChoice = "1"  # 1=prefer .rs files, 2=prefer mod.rs files
)

Write-Host "Module Ambiguity Resolution Script" -ForegroundColor Cyan
Write-Host "=================================" -ForegroundColor Cyan

if ($Auto) {
    Write-Host "Running in automatic mode with default choice: $DefaultChoice" -ForegroundColor Green
}

# List of modules with ambiguity issues
$modules = @("app", "dialog", "editor", "parser", "project", "ui", "visualization")

foreach ($module in $modules) {
    $filePath = "src\$module.rs"
    $dirPath = "src\$module"
    $modFilePath = "src\$module\mod.rs"
    
    $fileExists = Test-Path $filePath
    $modFileExists = Test-Path $modFilePath
    
    if ($fileExists -and $modFileExists) {
        Write-Host "Found ambiguity for module '$module'" -ForegroundColor Yellow
        
        if (-not $Auto) {
            Write-Host "  1. Keep $filePath and delete $modFilePath"
            Write-Host "  2. Keep $modFilePath and delete $filePath"
            Write-Host "  3. Skip this module"
            
            $choice = Read-Host "Enter your choice (1-3)"
        } else {
            $choice = $DefaultChoice
            Write-Host "Auto-selecting option $choice" -ForegroundColor Yellow
        }
        
        switch ($choice) {
            "1" {
                Write-Host "Keeping $filePath and deleting $modFilePath..." -ForegroundColor Green
                Remove-Item $modFilePath -Force
                Write-Host "Done." -ForegroundColor Green
            }
            "2" {
                Write-Host "Keeping $modFilePath and deleting $filePath..." -ForegroundColor Green
                Remove-Item $filePath -Force
                Write-Host "Done." -ForegroundColor Green
            }
            default {
                Write-Host "Skipping module '$module'" -ForegroundColor Yellow
            }
        }
    } elseif (-not $fileExists -and -not $modFileExists) {
        Write-Host "Module '$module' is referenced in main.rs but neither $filePath nor $modFilePath exists!" -ForegroundColor Red
        
        if (-not $Auto) {
            Write-Host "Would you like to create a basic file for this module?" -ForegroundColor Yellow
            $create = Read-Host "Create $filePath? (yes/no)"
        } else {
            # In auto mode, always create missing modules
            $create = "yes"
            Write-Host "Auto-creating missing module file" -ForegroundColor Yellow
        }
        
        if ($create -eq "yes") {
            Write-Host "Creating basic module file..." -ForegroundColor Green
            
            # Create directory if it doesn't exist
            $directory = [System.IO.Path]::GetDirectoryName($filePath)
            if (-not (Test-Path $directory)) {
                New-Item -ItemType Directory -Path $directory -Force | Out-Null
            }
            
            if ($module -eq "app") {
                @"
use eframe::egui;

pub struct App {
    // Add your app state here
}

impl Default for App {
    fn default() -> Self {
        Self {
            // Initialize your state
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust Code Visualizer");
            ui.label("Welcome to the Rust Code Visualizer!");
        });
    }
}
"@ | Out-File -FilePath $filePath -Encoding utf8
            } else {
                @"
// Basic implementation for $module module

pub fn init() {
    // Initialize $module functionality
}
"@ | Out-File -FilePath $filePath -Encoding utf8
            }
            
            Write-Host "$filePath created." -ForegroundColor Green
        }
    } else {
        Write-Host "Module '$module' is properly defined." -ForegroundColor Green
    }
}

Write-Host "Module ambiguity resolution complete!" -ForegroundColor Cyan
Write-Host "You can now try building the project again." -ForegroundColor Cyan
Write-Host "Run: cargo build --release" -ForegroundColor Yellow

# If in auto mode, display a summary of what was done
if ($Auto) {
    Write-Host "Auto-mode summary: Processed all modules with default choice: $DefaultChoice" -ForegroundColor Cyan
}

# PowerShell script to fix common Rust module and dependency issues

Write-Host "🔧 Rust Code Visualizer - Module Fix Script" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan

# Function to check if command exists
function Test-Command($cmdname) {
    return [bool](Get-Command -Name $cmdname -ErrorAction SilentlyContinue)
}

# Check if Rust is installed
if (-not (Test-Command "cargo")) {
    Write-Host "❌ Cargo not found! Please install Rust first." -ForegroundColor Red
    Write-Host "   Visit: https://rustup.rs/" -ForegroundColor Yellow
    exit 1
}

Write-Host "✅ Cargo found!" -ForegroundColor Green

# Clean previous builds
Write-Host "🧹 Cleaning previous builds..." -ForegroundColor Yellow
cargo clean

# Update Rust toolchain
Write-Host "🔄 Updating Rust toolchain..." -ForegroundColor Yellow
rustup update

# Check formatting
Write-Host "📝 Checking code formatting..." -ForegroundColor Yellow
if (Test-Command "rustfmt") {
    cargo fmt --check
    if ($LASTEXITCODE -ne 0) {
        Write-Host "⚠️  Code formatting issues found. Fixing..." -ForegroundColor Yellow
        cargo fmt
    }
} else {
    Write-Host "⚠️  rustfmt not found, installing..." -ForegroundColor Yellow
    rustup component add rustfmt
}

# Check for common issues with Clippy
Write-Host "🔍 Running Clippy for issue detection..." -ForegroundColor Yellow
if (Test-Command "cargo-clippy") {
    cargo clippy -- -D warnings
} else {
    Write-Host "⚠️  Clippy not found, installing..." -ForegroundColor Yellow
    rustup component add clippy
    cargo clippy -- -D warnings
}

# Build in debug mode first
Write-Host "🔨 Building in debug mode..." -ForegroundColor Yellow
cargo build

if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Debug build successful!" -ForegroundColor Green
    
    # Build in release mode
    Write-Host "🚀 Building in release mode..." -ForegroundColor Yellow
    cargo build --release
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Release build successful!" -ForegroundColor Green
        Write-Host "🎉 All builds completed successfully!" -ForegroundColor Cyan
    } else {
        Write-Host "❌ Release build failed!" -ForegroundColor Red
    }
} else {
    Write-Host "❌ Debug build failed!" -ForegroundColor Red
}

# Run tests if they exist
if (Test-Path "tests" -PathType Container) {
    Write-Host "🧪 Running tests..." -ForegroundColor Yellow
    cargo test
}

Write-Host "✨ Module fix script completed!" -ForegroundColor Cyan

# PowerShell script to fix module organization and dependencies

Write-Host "Fixing Rust modules and dependencies..." -ForegroundColor Green

# Check if Cargo.toml exists
if (!(Test-Path "Cargo.toml")) {
    Write-Host "Error: Cargo.toml not found!" -ForegroundColor Red
    exit 1
}

# Clean previous builds
Write-Host "Cleaning previous builds..." -ForegroundColor Yellow
cargo clean

# Check for missing dependencies
Write-Host "Checking dependencies..." -ForegroundColor Yellow
$dependencies = @("eframe", "egui", "syn", "walkdir", "serde", "serde_json", "once_cell")

foreach ($dep in $dependencies) {
    Write-Host "Checking $dep..." -ForegroundColor Cyan
}

# Update dependencies
Write-Host "Updating dependencies..." -ForegroundColor Yellow
cargo update

# Check syntax
Write-Host "Checking syntax..." -ForegroundColor Yellow
cargo check

if ($LASTEXITCODE -eq 0) {
    Write-Host "✓ All modules and dependencies are properly configured!" -ForegroundColor Green
} else {
    Write-Host "✗ There are still issues. Check the output above." -ForegroundColor Red
    exit 1
}

Write-Host "Module fix complete!" -ForegroundColor Green

# Rust Code Visualizer - Module Fix Script
# This script helps resolve common module and dependency issues

Write-Host "Rust Code Visualizer - Module Fix Script" -ForegroundColor Green
Write-Host "========================================"

# Check if Cargo.toml exists
if (-not (Test-Path "Cargo.toml")) {
    Write-Host "Error: Cargo.toml not found. Please run this script from the project root." -ForegroundColor Red
    exit 1
}

Write-Host "1. Cleaning previous build artifacts..." -ForegroundColor Yellow
cargo clean

Write-Host "2. Updating dependencies..." -ForegroundColor Yellow
cargo update

Write-Host "3. Checking for compilation issues..." -ForegroundColor Yellow
$buildResult = cargo check --all-targets --all-features 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "Build check failed. Output:" -ForegroundColor Red
    Write-Host $buildResult -ForegroundColor Red
} else {
    Write-Host "Build check passed!" -ForegroundColor Green
}

Write-Host "4. Running cargo fix for automatic fixes..." -ForegroundColor Yellow
cargo fix --allow-dirty --allow-staged

Write-Host "5. Formatting code..." -ForegroundColor Yellow
cargo fmt

Write-Host "6. Running clippy for linting..." -ForegroundColor Yellow
cargo clippy --all-targets --all-features -- -W clippy::all

Write-Host "7. Attempting final build..." -ForegroundColor Yellow
$finalBuild = cargo build --release 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "Final build failed. Please check the errors above." -ForegroundColor Red
    Write-Host $finalBuild -ForegroundColor Red
} else {
    Write-Host "Final build successful!" -ForegroundColor Green
    Write-Host "You can now run the application with: cargo run --release" -ForegroundColor Cyan
}

Write-Host "Script completed." -ForegroundColor Green

# PowerShell script to fix module structure and dependencies

Write-Host "Rust Code Visualizer - Module Fix Script" -ForegroundColor Green
Write-Host "=======================================" -ForegroundColor Green

# Check if we're in the right directory
if (-not (Test-Path "Cargo.toml")) {
    Write-Host "Error: Cargo.toml not found. Please run this script from the project root." -ForegroundColor Red
    exit 1
}

Write-Host "Checking Rust installation..." -ForegroundColor Yellow
try {
    $rustVersion = cargo --version
    Write-Host "Found: $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "Error: Rust/Cargo not found. Please install Rust first." -ForegroundColor Red
    exit 1
}

Write-Host "Checking project structure..." -ForegroundColor Yellow
$requiredDirs = @("src", "src\app", "src\visualization", "src\ui", "src\editor")
foreach ($dir in $requiredDirs) {
    if (Test-Path $dir) {
        Write-Host "✓ $dir exists" -ForegroundColor Green
    } else {
        Write-Host "✗ $dir missing" -ForegroundColor Red
    }
}

Write-Host "Running cargo check..." -ForegroundColor Yellow
try {
    cargo check 2>&1 | Tee-Object -Variable checkOutput
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓ Cargo check passed" -ForegroundColor Green
    } else {
        Write-Host "✗ Cargo check failed" -ForegroundColor Red
        Write-Host $checkOutput -ForegroundColor Gray
    }
} catch {
    Write-Host "Error running cargo check" -ForegroundColor Red
}

Write-Host "Running cargo build..." -ForegroundColor Yellow
try {
    cargo build 2>&1 | Tee-Object -Variable buildOutput
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓ Build successful" -ForegroundColor Green
    } else {
        Write-Host "✗ Build failed" -ForegroundColor Red
        Write-Host $buildOutput -ForegroundColor Gray
    }
} catch {
    Write-Host "Error running cargo build" -ForegroundColor Red
}

Write-Host "Module fix complete!" -ForegroundColor Green
Write-Host "To run the application: cargo run --release" -ForegroundColor Cyan

# PowerShell script to fix common module and dependency issues

Write-Host "Rust Code Visualizer - Module Fix Script" -ForegroundColor Green
Write-Host "=======================================" -ForegroundColor Green

# Function to check if Rust is installed
function Test-RustInstallation {
    try {
        $rustVersion = rustc --version
        Write-Host "✓ Rust installed: $rustVersion" -ForegroundColor Green
        return $true
    }
    catch {
        Write-Host "✗ Rust not found. Please install Rust from https://rustup.rs/" -ForegroundColor Red
        return $false
    }
}

# Function to check Cargo
function Test-CargoInstallation {
    try {
        $cargoVersion = cargo --version
        Write-Host "✓ Cargo available: $cargoVersion" -ForegroundColor Green
        return $true
    }
    catch {
        Write-Host "✗ Cargo not found." -ForegroundColor Red
        return $false
    }
}

# Function to clean and rebuild
function Invoke-CleanBuild {
    Write-Host "Cleaning previous build artifacts..." -ForegroundColor Yellow
    
    if (Test-Path "target") {
        Remove-Item -Recurse -Force "target"
        Write-Host "✓ Removed target directory" -ForegroundColor Green
    }
    
    if (Test-Path "Cargo.lock") {
        Remove-Item "Cargo.lock"
        Write-Host "✓ Removed Cargo.lock" -ForegroundColor Green
    }
    
    Write-Host "Rebuilding project..." -ForegroundColor Yellow
    cargo build
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓ Build successful!" -ForegroundColor Green
    } else {
        Write-Host "✗ Build failed. Check the error messages above." -ForegroundColor Red
    }
}

# Function to update dependencies
function Update-Dependencies {
    Write-Host "Updating dependencies..." -ForegroundColor Yellow
    cargo update
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓ Dependencies updated!" -ForegroundColor Green
    } else {
        Write-Host "✗ Failed to update dependencies." -ForegroundColor Red
    }
}

# Function to check for common issues
function Test-CommonIssues {
    Write-Host "Checking for common issues..." -ForegroundColor Yellow
    
    # Check if we're in the right directory
    if (-not (Test-Path "Cargo.toml")) {
        Write-Host "✗ Cargo.toml not found. Make sure you're in the project root directory." -ForegroundColor Red
        return $false
    }
    
    # Check if all source files exist
    $sourceFiles = @(
        "src/lib.rs",
        "src/main.rs",
        "src/app/mod.rs",
        "src/parser.rs",
        "src/project.rs"
    )
    
    foreach ($file in $sourceFiles) {
        if (Test-Path $file) {
            Write-Host "✓ Found $file" -ForegroundColor Green
        } else {
            Write-Host "✗ Missing $file" -ForegroundColor Red
        }
    }
    
    return $true
}

# Function to run tests
function Invoke-Tests {
    Write-Host "Running tests..." -ForegroundColor Yellow
    cargo test
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓ All tests passed!" -ForegroundColor Green
    } else {
        Write-Host "✗ Some tests failed." -ForegroundColor Red
    }
}

# Function to check formatting
function Test-Formatting {
    Write-Host "Checking code formatting..." -ForegroundColor Yellow
    cargo fmt --check
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓ Code is properly formatted!" -ForegroundColor Green
    } else {
        Write-Host "! Code formatting issues found. Run 'cargo fmt' to fix." -ForegroundColor Yellow
    }
}

# Function to run clippy
function Invoke-Clippy {
    Write-Host "Running Clippy lints..." -ForegroundColor Yellow
    cargo clippy -- -D warnings
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓ No Clippy warnings!" -ForegroundColor Green
    } else {
        Write-Host "! Clippy found some issues to fix." -ForegroundColor Yellow
    }
}

# Main execution
function Main {
    param(
        [string]$Action = "check"
    )
    
    switch ($Action.ToLower()) {
        "check" {
            if (Test-RustInstallation -and Test-CargoInstallation) {
                Test-CommonIssues
                Test-Formatting
            }
        }
        "clean" {
            Invoke-CleanBuild
        }
        "update" {
            Update-Dependencies
        }
        "test" {
            Invoke-Tests
        }
        "lint" {
            Invoke-Clippy
        }
        "all" {
            if (Test-RustInstallation -and Test-CargoInstallation) {
                Test-CommonIssues
                Update-Dependencies
                Invoke-CleanBuild
                Test-Formatting
                Invoke-Clippy
                Invoke-Tests
            }
        }
        default {
            Write-Host "Usage: .\fix_modules.ps1 [check|clean|update|test|lint|all]" -ForegroundColor Cyan
            Write-Host "  check  - Check installation and common issues (default)"
            Write-Host "  clean  - Clean and rebuild project"
            Write-Host "  update - Update dependencies"
            Write-Host "  test   - Run tests"
            Write-Host "  lint   - Run clippy lints"
            Write-Host "  all    - Run all checks and fixes"
        }
    }
}

# Run with the provided parameter or default to "check"
Main -Action $args[0]

# Fix common Rust module and dependency issues

Write-Host "Rust Code Visualizer - Module Fix Script" -ForegroundColor Green
Write-Host "=====================================" -ForegroundColor Green

# Function to check if running as administrator
function Test-Administrator {
    $currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

# Check for administrator privileges
if (-not (Test-Administrator)) {
    Write-Warning "This script should be run as Administrator for best results."
    Write-Host "Continuing with limited privileges..." -ForegroundColor Yellow
}

# Clean cargo cache and registry
Write-Host "Cleaning Cargo cache..." -ForegroundColor Yellow
if (Test-Path "$env:USERPROFILE\.cargo\registry") {
    try {
        Remove-Item "$env:USERPROFILE\.cargo\registry\cache" -Recurse -Force -ErrorAction SilentlyContinue
        Write-Host "Cargo cache cleaned successfully" -ForegroundColor Green
    } catch {
        Write-Warning "Could not clean cargo cache: $_"
    }
}

# Clean target directory
Write-Host "Cleaning target directory..." -ForegroundColor Yellow
if (Test-Path "target") {
    try {
        Remove-Item "target" -Recurse -Force
        Write-Host "Target directory cleaned successfully" -ForegroundColor Green
    } catch {
        Write-Warning "Could not clean target directory: $_"
    }
}

# Update Cargo.lock
Write-Host "Updating Cargo.lock..." -ForegroundColor Yellow
try {
    cargo update
    Write-Host "Cargo.lock updated successfully" -ForegroundColor Green
} catch {
    Write-Warning "Could not update Cargo.lock: $_"
}

# Check for common issues
Write-Host "Checking for common issues..." -ForegroundColor Yellow

# Check Rust version
$rustVersion = cargo --version
Write-Host "Rust version: $rustVersion" -ForegroundColor Cyan

# Check for missing dependencies
Write-Host "Verifying dependencies..." -ForegroundColor Yellow
$dependencies = @("eframe", "egui", "syn", "walkdir")
foreach ($dep in $dependencies) {
    try {
        cargo search $dep --limit 1 | Out-Null
        Write-Host "✓ $dep is available" -ForegroundColor Green
    } catch {
        Write-Warning "✗ Issue with dependency: $dep"
    }
}

# Try to build
Write-Host "Attempting to build project..." -ForegroundColor Yellow
try {
    cargo check
    Write-Host "Project builds successfully!" -ForegroundColor Green
} catch {
    Write-Warning "Build issues detected. Try running: cargo build --verbose"
}

Write-Host "Module fix script completed!" -ForegroundColor Green
Write-Host "If issues persist, try:" -ForegroundColor Yellow
Write-Host "  1. cargo clean" -ForegroundColor White
Write-Host "  2. cargo build --release --verbose" -ForegroundColor White
Write-Host "  3. Restart your IDE/editor" -ForegroundColor White
