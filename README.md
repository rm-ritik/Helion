# Helion

High-performance data visualization engine powered by WebGPU. Render millions of data points at 60 FPS where traditional libraries struggle.

## Project Scope

Helion uses WebGPU and Rust to deliver GPU-accelerated data visualization for the web. Built for financial dashboards, IoT monitoring, and scientific data exploration that exceed the limits of D3.js and Plotly.

## Version 1.0 Goals

**Core Engine**
- Rust rendering engine with WebGPU backend
- Scatter plots and line charts
- Handle 1M+ data points at 60 FPS

**Language Bindings**
- Python with Jupyter notebook integration
- TypeScript API and React components

**Features**
- Interactive pan/zoom
- Basic styling (colors, point sizes)
- Export to static formats

## Project Structure

```
helion/
├── core/              # Rust + WebGPU rendering
├── bindings/
│   ├── python/        # PyO3 bindings
│   └── typescript/    # WASM bindings
└── examples/          # Demo applications
```

## License

Apache 2.0
