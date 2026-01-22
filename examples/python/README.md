# Python Examples

These examples demonstrate how to use Helion's Python API for creating high-performance data visualizations.

## Prerequisites

### Quick Setup (Recommended)

**From project root, first time setup:**
```bash
cd bindings/python
./setup.sh
```

**Or if already set up, just rebuild:**
```bash
cd bindings/python
./build.sh
```

### Manual Setup

If you prefer to install manually:
```bash
# From project root

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install Python dependencies and build
cd bindings/python
pip install maturin numpy
maturin develop --release
```

## Running Examples

Make sure the virtual environment is activated:
```bash
# From project root
source .venv/bin/activate
cd examples/python

# Run any example
python scatter_basic.py
```

## Examples

### 1. Basic Scatter Plot (`scatter_basic.py`)
Simple scatter plot with 100,000 random points.

```bash
python scatter_basic.py
```

### 2. Million Points (`scatter_million.py`)
Performance test with 1 million data points demonstrating Helion's GPU acceleration capabilities.

```bash
python scatter_million.py
```

### 3. Color Options (`scatter_colors.py`)
Shows different ways to specify colors: hex strings, RGB tuples, and the Color class.

```bash
python scatter_colors.py
```

### 4. Custom Ranges (`scatter_custom_ranges.py`)
Demonstrates custom coordinate mapping with `x_range` and `y_range` parameters for different output spaces.

```bash
python scatter_custom_ranges.py
```

## Expected Output

When running these examples, you should see:
- Fast data processing times (even with 1M points)
- Confirmation that the scatter plot was created
- Statistics about your data

Note: These examples currently create the scatter plot data structures but don't render to screen yet. Full rendering will be available once we add window/canvas integration.
