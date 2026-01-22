# Helion

High-performance data visualization engine powered by WebGPU. Render millions of data points at 60 FPS where traditional libraries struggle.

## 📚 Documentation

- **[API Documentation](https://rm-ritik.github.io/Helion/helion_core/)** - Full API reference
- **[Repository](https://github.com/rm-ritik/Helion)** - Source code
- **[Python Examples](examples/python/)** - Working examples with setup instructions

## Project Scope

Helion uses WebGPU and Rust to deliver GPU-accelerated data visualization for the web and desktop (Windows, macOS, Linux). Built for financial dashboards, IoT monitoring, and scientific data exploration that exceed the limits of D3.js and Plotly.

## Version 0.1.0 Goals

**Core Engine**
- Rust rendering engine with WebGPU backend
- WebGL 2.0 fallback for broader compatibility
- Scatter plots and line charts
- Handle 1M+ data points at 60 FPS

**Language Bindings**
- Python with Jupyter notebook integration (PyO3) - Windows, macOS, Linux
- Framework-agnostic JavaScript/TypeScript API
- Thin React/Vue/Svelte wrappers

**Features**
- Interactive pan/zoom
- Basic styling (colors, point sizes)
- Automatic GPU backend detection
- Optimized WASM bundle (<500KB)

## Quick Start (Python)

Helion's Python bindings let you visualize massive datasets with GPU acceleration on Windows, macOS, and Linux. All examples open native windows to display plots.

### Examples

- **[scatter_basic.py](examples/python/scatter_basic.py)** - Simple scatter plot with 100K points
- **[scatter_million.py](examples/python/scatter_million.py)** - Performance test with 1M points (renders smoothly at 60 FPS!)
- **[scatter_colors.py](examples/python/scatter_colors.py)** - Different color formats (hex, RGB tuples, Color class)
- **[scatter_custom_ranges.py](examples/python/scatter_custom_ranges.py)** - Custom coordinate mapping

**Setup & Installation:** See [bindings/python/README.md](bindings/python/README.md)

```python
import helion
import numpy as np

# Generate 1 million points
x = np.random.rand(1_000_000)
y = np.random.rand(1_000_000)

# Create and display scatter plot - renders smoothly at 60 FPS!
plot = helion.scatter(x, y, color="#FF5733")
plot.show()  # Opens a window with GPU-accelerated rendering
```

## Project Structure

```
helion/
├── core/              # Rust + WebGPU/WebGL2
│   ├── src/           # Core rendering engine
│   └── tests/         # Integration tests
├── bindings/
│   ├── python/        # PyO3 bindings + Jupyter
│   │   ├── src/       # Rust FFI bindings
│   │   └── tests/     # Unit tests
│   └── web/           # WASM → JS/TS
│       ├── vanilla/   # Framework-agnostic core
│       └── react/     # React components
└── examples/          # Demo applications
    └── python/        # ✅ Working Python examples
```

## License

Apache 2.0
