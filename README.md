# Helion

High-performance data visualization engine powered by WebGPU. Render millions of data points at 60 FPS where traditional libraries struggle.

## 📚 Documentation

- **[API Documentation](https://rm-ritik.github.io/Helion/helion_core/)** - Full API reference
- **[Repository](https://github.com/rm-ritik/Helion)** - Source code

## Project Scope

Helion uses WebGPU and Rust to deliver GPU-accelerated data visualization for the web. Built for financial dashboards, IoT monitoring, and scientific data exploration that exceed the limits of D3.js and Plotly.

## Version 1.0 Goals

**Core Engine**
- Rust rendering engine with WebGPU backend
- WebGL 2.0 fallback for broader compatibility
- Scatter plots and line charts
- Handle 1M+ data points at 60 FPS

**Language Bindings**
- Python with Jupyter notebook integration (PyO3)
- Framework-agnostic JavaScript/TypeScript API
- Thin React/Vue/Svelte wrappers

**Features**
- Interactive pan/zoom
- Basic styling (colors, point sizes)
- Automatic GPU backend detection
- Optimized WASM bundle (<500KB)

## Project Structure

```
helion/
├── core/              # Rust + WebGPU/WebGL2
├── bindings/
│   ├── python/        # PyO3 bindings + Jupyter
│   └── web/           # WASM → JS/TS
│       ├── vanilla/   # Framework-agnostic core
│       └── react/     # React components
└── examples/          # Demo applications
```

## License

Apache 2.0
