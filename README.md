# Rust Code Visualizer

A visualization tool for Rust codebases that helps understand project structure and code relationships.

## Features

- Project file browser
- Code visualization with multiple layout algorithms
- Syntax-highlighted code editor
- Relationship analysis between code elements

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run --release
```

## Usage

1. Launch the application
2. Use File -> Open Project to select a Rust project directory
3. Switch between Visualization and Editor views using the top menu
4. Explore your code structure visually or edit files directly

## Layout Types

- **Force Directed**: Elements are positioned using physics simulation
- **Grid**: Elements arranged in a regular grid pattern  
- **Circular**: Elements positioned in circular patterns
- **Tree**: Hierarchical tree layout
- **Hierarchical**: Multi-level hierarchical arrangement

## Controls

- **Mouse wheel**: Zoom in/out in visualization view
- **Click and drag**: Pan around the visualization
- **Click elements**: Select and highlight relationships
