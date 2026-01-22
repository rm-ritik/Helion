#!/bin/bash
# Helion Python Bindings - Complete Setup Script
#
# PURPOSE:
#   Full installation script for fresh machines. Checks and installs ALL dependencies
#   including Rust toolchain, Python packages, and builds Helion from scratch.
#
# WHEN TO USE:
#   - First time setup on a new machine
#   - Clean installation after system wipe
#   - Setting up CI/CD environments
#   - When Rust is not installed
#   - When you want to verify all dependencies are correct
#
# WHAT IT DOES:
#   1. Checks Python version (requires 3.10+)
#   2. Installs Rust toolchain if missing (via rustup)
#   3. Creates Python virtual environment
#   4. Installs maturin and numpy
#   5. Builds and installs Helion
#   6. Verifies installation works
#
# USAGE:
#   cd bindings/python
#   ./setup.sh
#
# AFTER FIRST SETUP:
#   For faster rebuilds during development, use build.sh instead.
#
# TIME ESTIMATE:
#   - With Rust already installed: ~2-3 minutes
#   - Fresh install (including Rust): ~5-10 minutes

set -e  # Exit on error

echo "================================================"
echo "Helion Python Bindings - Setup Script"
echo "================================================"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Change to script directory
cd "$(dirname "$0")"

# 1. Check Python version
echo "1. Checking Python version..."
PYTHON_VERSION=$(python3 --version 2>&1 | awk '{print $2}')
PYTHON_MAJOR=$(echo $PYTHON_VERSION | cut -d. -f1)
PYTHON_MINOR=$(echo $PYTHON_VERSION | cut -d. -f2)

if [ "$PYTHON_MAJOR" -lt 3 ] || [ "$PYTHON_MAJOR" -eq 3 -a "$PYTHON_MINOR" -lt 10 ]; then
    echo -e "${RED}✗ Python 3.10+ required. Found: $PYTHON_VERSION${NC}"
    echo "Please install Python 3.10 or higher from python.org"
    exit 1
fi
echo -e "${GREEN}✓ Python $PYTHON_VERSION found${NC}"
echo ""

# 2. Check/Install Rust
echo "2. Checking Rust installation..."
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    echo -e "${GREEN}✓ Rust already installed: $RUST_VERSION${NC}"
else
    echo -e "${YELLOW}⚠ Rust not found. Installing...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo -e "${GREEN}✓ Rust installed successfully${NC}"
fi
echo ""

# Ensure cargo is in PATH
if ! command -v cargo &> /dev/null; then
    export PATH="$HOME/.cargo/bin:$PATH"
    source "$HOME/.cargo/env" 2>/dev/null || true
fi

# 3. Setup Python virtual environment
echo "3. Setting up Python virtual environment..."
VENV_PATH="../../.venv"

if [ -d "$VENV_PATH" ]; then
    echo -e "${GREEN}✓ Virtual environment already exists${NC}"
else
    echo "Creating virtual environment..."
    python3 -m venv "$VENV_PATH"
    echo -e "${GREEN}✓ Virtual environment created${NC}"
fi

# Activate virtual environment
source "$VENV_PATH/bin/activate"
echo -e "${GREEN}✓ Virtual environment activated${NC}"
echo ""

# 4. Install Python dependencies
echo "4. Installing Python dependencies..."
pip install --upgrade pip > /dev/null 2>&1
echo "   Installing maturin..."
pip install maturin > /dev/null 2>&1
echo "   Installing numpy..."
pip install numpy > /dev/null 2>&1
echo -e "${GREEN}✓ Dependencies installed${NC}"
echo ""

# 5. Build Helion Python bindings
echo "5. Building Helion Python bindings..."
echo "   This may take a few minutes on first build..."

# Unset CONDA_PREFIX to avoid conflicts
unset CONDA_PREFIX

# Build with maturin
VIRTUAL_ENV=$VENV_PATH maturin develop --release

echo ""
echo -e "${GREEN}✓ Build complete!${NC}"
echo ""

# 6. Verify installation
echo "6. Verifying installation..."
python3 -c "import helion; print(f'Helion version: {helion.__version__}')" 2>&1
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Helion imported successfully!${NC}"
else
    echo -e "${RED}✗ Import failed${NC}"
    exit 1
fi
echo ""

# Success message
echo "================================================"
echo -e "${GREEN}Setup Complete!${NC}"
echo "================================================"
echo ""
echo "To use Helion:"
echo "  1. Activate the virtual environment:"
echo "     source ../../.venv/bin/activate"
echo ""
echo "  2. Try an example:"
echo "     cd ../../examples/python"
echo "     python scatter_basic.py"
echo ""
echo "  3. Or use in your own code:"
echo "     import helion"
echo "     import numpy as np"
echo "     x = np.random.rand(100000)"
echo "     y = np.random.rand(100000)"
echo "     plot = helion.scatter(x, y)"
echo ""
