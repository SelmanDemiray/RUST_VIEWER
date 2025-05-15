# Rust Code Visualizer

A tool for visualizing Rust code projects, allowing you to see relationships between files and code elements with a graphical interface.

## Features

- Upload and browse Rust project files
- Visualize code structure with graphical representation
- See relationships between code elements (functions, modules, etc.)
- Interactive code editor for viewing and modifying code
- Visual arrows showing connections between related code elements

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.81.0 or newer)
- Cargo (comes with Rust)
- Git (optional, for cloning the repository)

## Installation

### Rust Version Requirement

This project is configured to work with Rust 1.81.0. If you have a newer version, you should have no issues. If you have an older version, please update your Rust installation:

```bash
rustup update stable
```

### Clone or Download

```bash
# Clone the repository
git clone https://github.com/yourusername/rust_code_visualizer.git
# Or download and extract the ZIP file

# Navigate to the project directory
cd rust_code_visualizer
```

### Build from Source

```bash
# Build the project
cargo build --release
```

The compiled executable will be available in `target/release/rust_code_visualizer`.

## Running the Application

### Using Cargo

```bash
cargo run --release
```

### Using the Executable

After building, you can run the executable directly:

```bash
# On Windows
.\target\release\rust_code_visualizer.exe

# On Linux/macOS
./target/release/rust_code_visualizer
```

## Usage Guide

### Opening a Project

1. Launch the application
2. Click the "Open Project" button in the top panel
3. Navigate to and select the root folder of your Rust project
4. The application will scan and display the project files

### Viewing Files

- The left panel shows all the files in your project
- Click on a file to select it for viewing

### Visualization Mode

- Click on "Visualization" in the top panel to switch to visualization mode
- Files are shown as rectangles
- Code elements (functions, modules, etc.) are shown as smaller shapes
- Arrows indicate relationships between code elements
- You can drag and zoom in the visualization area for better viewing

### Editor Mode

- Click on "Editor" in the top panel to switch to editor mode
- Select a file from the left panel to view its contents in the editor
- The editor supports syntax highlighting for Rust code

## Troubleshooting

### Common Issues

- **Build errors**: Make sure you have at least Rust 1.81.0. Check your version with `rustc --version` and update if needed with `rustup update stable`
- **Missing dependencies**: Run `cargo check` to see if all dependencies can be resolved
- **Visualization not showing**: Some complex projects might take time to parse
- **Package not found errors**: If you see an error like `no matching package found` for `syntax_tree`, make sure you're using the correct package name with hyphens instead of underscores (e.g., `syntax-tree` instead of `syntax_tree`)
- **Version errors**: If you see an error like `failed to select a version for the requirement`, check that the version specified in Cargo.toml actually exists. You may need to use an older version like `0.3.2` instead of `0.4.0`.
- **Patching errors**: If you see errors about patch resolution or conflicts, try using direct versioning in the dependencies section instead of patches.
- **Rust version compatibility errors**: If you see errors about packages requiring a newer Rust version, try updating your Rust installation with `rustup update stable`.

### Getting Past Rust Version Requirements

If you're experiencing the error about ICU packages requiring Rust 1.82, you have two options:

1. **Upgrade Rust** (recommended): 
   ```bash
   rustup update stable
   ```

2. **Use the version-pinned approach**: We've modified the Cargo.toml file to use specific older versions of dependencies that are known to be compatible with Rust 1.81.0. This means:
   - The code editor functionality is simplified
   - Some features might be missing compared to newer versions
   - The application should now build on Rust 1.81.0

### Major Dependency Changes

To make the project work with Rust 1.81.0, we've made these changes:

1. Removed `egui_code_editor` and replaced it with a simple `TextEdit` from egui
2. Pinned all dependencies to specific older versions
3. Simplified the file dialog implementation to avoid pulling in ICU-related dependencies
4. Turned off certain optimizations to prevent complex dependency resolution issues

If you're still encountering issues, we strongly recommend upgrading to at least Rust 1.82.0.

### Major Changes to Fix Compatibility Issues

If you're still experiencing dependency issues with Rust 1.81.0, we've made the following changes to increase compatibility:

1. Removed the `rfd` dependency which was pulling in problematic ICU-related dependencies that required Rust 1.82
2. Implemented our own simple file dialog using pure egui/eframe that works with older Rust versions
3. Simplified the dependency tree to avoid conflicts with transitive dependencies

If you still encounter issues, you may need to upgrade your Rust installation to version 1.82.0 or newer:

```bash
rustup update stable
```

### Dependency Issues

If you encounter errors related to dependencies:
1. Check the exact package name in the [crates.io](https://crates.io) registry. Package names may use hyphens instead of underscores.
2. Verify that the version you're requesting exists by checking the package's page on crates.io.
3. Run `cargo update` to update the dependency to the latest compatible version.
4. Try using `default-features = false` for problematic dependencies and only enable specific features you need.
5. For transitive dependency conflicts, you may need to use simpler alternatives or remove functionality that depends on problematic packages.
6. If all else fails, try running `cargo build --release -Z minimal-versions` to use the lowest possible version of each dependency that still satisfies the requirements.

### Reporting Bugs

If you encounter any issues, please report them on the GitHub repository issue tracker:
https://github.com/yourusername/rust_code_visualizer/issues

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
