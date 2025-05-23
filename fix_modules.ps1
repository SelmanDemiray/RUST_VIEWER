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
