# Helion Python Bindings

High-performance data visualization engine powered by WebGPU.

## Installation

### Quick Install (Recommended)

**For first-time setup or new machines:**
```bash
cd bindings/python
./setup.sh
```

This script will:
- ✅ Check Python version (requires 3.10+)
- ✅ Install Rust toolchain if needed
- ✅ Create virtual environment
- ✅ Install maturin and numpy
- ✅ Build and install Helion
- ✅ Verify installation

**For rebuilding after code changes:**
```bash
cd bindings/python
./build.sh
```

This faster script assumes dependencies are already installed and just rebuilds the package.

| Script | When to Use | Prerequisites | Time |
|--------|-------------|---------------|------|
| `setup.sh` | New machine, first install | None | ~5-10 min |
| `build.sh` | Development, code changes | Rust installed | ~30 sec |

### Manual Installation

If you prefer manual setup:

### Prerequisites

1. **Python 3.10+**
   ```bash
   python --version  # Should be 3.10 or higher
   ```

2. **Rust toolchain**
   ```bash
   # Install Rust (if not already installed)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env  # Add Rust to PATH
   
   # Verify installation
   rustc --version
   cargo --version
   ```

3. **Maturin** (Python/Rust bridge tool)
   ```bash
   pip install maturin
   ```

4. **NumPy**
   ```bash
   pip install numpy
   ```

### Build and Install

```bash
# Navigate to bindings directory
cd bindings/python

# Development install (recommended for local development)
maturin develop --release

# Or build wheel for distribution
maturin build --release
pip install target/wheels/helion-*.whl
```

### Quick Install Script

For convenience, use the provided build script:
```bash
cd bindings/python
./build.sh  # Handles venv setup and build automatically
```

## Quick Start

```python
import helion
import numpy as np

# Generate random data (float64 is fine - auto-converted to float32)
x = np.random.rand(100000)
y = np.random.rand(100000)

# Create scatter plot
plot = helion.scatter(x, y, color="#FF5733")
```

**Note:** Helion uses float32 internally for optimal GPU performance. Your NumPy arrays are automatically converted.

## Features

- **GPU Acceleration**: WebGPU-powered rendering for millions of data points
- **NumPy Integration**: Seamless integration with NumPy arrays
- **Jupyter Support**: Works great in Jupyter notebooks
- **Easy to Use**: Simple, intuitive Python API

## Requirements

- Python >= 3.10
- NumPy >= 1.20

## Documentation

See the [examples](../../examples/python/) directory for usage examples.
