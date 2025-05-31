# Rust Code Visualizer

A powerful tool for visualizing Rust code structure and relationships, built with egui.

## Features

- **Interactive Visualization**: Multiple layout algorithms (Force-directed, Grid, Circular, Tree, Hierarchical)
- **Code Analysis**: Parse Rust files to extract functions, structs, enums, traits, and their relationships
- **Dual View Mode**: Switch between visualization and code editor views
- **Project Navigation**: Browse project files and explore code structure
- **Real-time Filtering**: Filter elements by name or file path
- **Relationship Mapping**: Visualize imports, implementations, and function calls

## Installation

### Prerequisites
- Rust 1.70 or later
- Windows, macOS, or Linux

### Building from Source

```bash
git clone <repository-url>
cd RUST_VIEWER
cargo build --release
```

## Usage

### Running the Application

```bash
cargo run --release
```

### Opening a Project

1. Click **File → Open Project** in the menu
2. Navigate to your Rust project directory
3. Select the directory containing your Rust source files
4. The visualizer will parse all `.rs` files and display the code structure

### Navigation

- **Zoom**: Use mouse wheel to zoom in/out
- **Pan**: Click and drag to move around the visualization
- **Select Elements**: Click on nodes to select and highlight relationships
- **Filter**: Use the filter box in the control panel to search for specific elements

### Layout Options

- **Force-Directed**: Physics-based layout with customizable forces
- **Grid**: Organized grid layout
- **Circular**: Circular arrangement of files
- **Tree**: Hierarchical tree structure
- **Hierarchical**: Directory-based hierarchy

### View Modes

- **Visualization**: Interactive graph view of code structure
- **Editor**: Syntax-highlighted code viewer with line numbers

## Project Structure

```
src/
├── main.rs              # Application entry point
├── lib.rs               # Library exports
├── app/                 # Application state and logic
│   ├── mod.rs
│   ├── state.rs
│   └── view_mode.rs
├── project/             # Project loading and management
│   └── mod.rs
├── parser/              # Rust code parsing
│   └── mod.rs
├── visualization/       # Visualization engine
│   ├── mod.rs
│   ├── state.rs
│   ├── renderer.rs
│   ├── layout/          # Layout algorithms
│   │   ├── mod.rs
│   │   ├── force_directed.rs
│   │   ├── grid.rs
│   │   ├── circular.rs
│   │   ├── tree.rs
│   │   └── hierarchical.rs
│   └── components/      # UI components
│       ├── mod.rs
│       ├── elements.rs
│       ├── relationships.rs
│       ├── minimap.rs
│       └── status_bar.rs
├── ui/                  # User interface
│   ├── mod.rs
│   ├── top_panel.rs
│   ├── side_panel.rs
│   └── central_panel.rs
├── editor/              # Code editor
│   ├── mod.rs
│   └── renderer.rs
├── dialog.rs            # File dialogs
└── simple_dialog.rs     # Simple file picker
```

## Configuration

The application uses several configuration files:

### .cargo/config.toml
Contains build optimization settings for Windows to avoid file locking issues.

## Troubleshooting

### Build Issues

If you encounter "Access is denied" errors during build:

1. Run the cleanup script: `.\cleanup.ps1`
2. Try building with a single job: `cargo build -j 1`
3. Ensure no antivirus software is scanning the project directory

### Performance

For large projects:
- Use the Grid or Hierarchical layout for better performance
- Adjust the zoom level to reduce rendering overhead
- Filter elements to focus on specific parts of the codebase

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with [egui](https://github.com/emilk/egui) - Immediate mode GUI framework
- Rust parsing powered by [syn](https://github.com/dtolnay/syn)
- File system traversal using [walkdir](https://github.com/BurntSushi/walkdir)

## Roadmap

- [ ] Support for more Rust constructs (macros, attributes)
- [ ] Export visualizations as images
- [ ] Plugin system for custom layouts
- [ ] Performance optimizations for very large codebases
- [ ] Integration with Cargo workspaces
- [ ] Call graph analysis
- [ ] Dependency visualization
