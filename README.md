# Rust Code Visualizer

A visual tool for exploring and understanding Rust codebases through interactive diagrams and visualizations.

## Features

- **Interactive Visualization**: Explore your Rust code through force-directed graphs, hierarchical layouts, and more
- **Multiple Layout Algorithms**: Choose from force-directed, grid, circular, tree, and hierarchical layouts
- **Code Editor Integration**: View and edit source files within the application
- **Project Navigation**: Browse project files and understand code relationships
- **Cross-platform Support**: Runs on Windows, macOS, and Linux

## Building

### Prerequisites
- Rust 1.70+ 
- For Windows cross-compilation: mingw-w64

### Build Commands

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Windows cross-compilation (from Linux)
cargo build --release --target x86_64-pc-windows-gnu
```

## Usage

1. Run the application: `cargo run` or `./target/release/rust_code_visualizer`
2. Click "Open Project" to select a Rust project directory
3. Explore the visualization using different layout algorithms
4. Click on elements to view details and relationships

## Architecture

- `app/`: Application state and main loop
- `parser/`: Rust code parsing and analysis
- `project/`: Project loading and management
- `visualization/`: Graph visualization and rendering
- `editor/`: Code editor component
- `ui/`: User interface components

## License

MIT License
