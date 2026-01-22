#!/bin/bash
# Build script for Helion Python bindings
#
# PURPOSE:
#   Quick rebuild script for development. Assumes Rust and basic dependencies
#   are already installed. Recompiles the Rust code and reinstalls the Python package.
#
# WHEN TO USE:
#   - After making changes to Rust code (core/ or bindings/python/src/)
#   - When switching git branches
#   - To rebuild after pulling new changes
#   - During active development (fast iteration)
#
# REQUIREMENTS:
#   - Rust toolchain already installed
#   - Maturin and numpy in venv (or will be installed)
#   - Virtual environment at ../../.venv (or will be created)
#
# USAGE:
#   cd bindings/python
#   ./build.sh
#
# FIRST TIME SETUP:
#   If this is a completely new machine, use setup.sh instead.
#   This script assumes Rust is already installed.

set -e

cd "$(dirname "$0")"

echo "Building Helion Python bindings..."
echo "===================================="

# Ensure we're using the venv Python
if [ -z "$VIRTUAL_ENV" ]; then
    VENV_PATH="$(pwd)/../../.venv"
    if [ -d "$VENV_PATH" ]; then
        source "$VENV_PATH/bin/activate"
        echo "✓ Activated virtual environment"
    else
        echo "⚠️  No virtual environment found. Creating one..."
        python3 -m venv "$VENV_PATH"
        source "$VENV_PATH/bin/activate"
        pip install maturin numpy
        echo "✓ Created and activated virtual environment"
    fi
fi

# Unset CONDA_PREFIX to avoid conflicts
unset CONDA_PREFIX

# Build with maturin
echo ""
echo "Building with maturin..."
maturin develop

echo ""
echo "===================================="
echo "✓ Build complete!"
echo ""
echo "To use Helion:"
echo "  source ../../.venv/bin/activate"
echo "  python"
echo "  >>> import helion"
echo "  >>> import numpy as np"
echo "  >>> plot = helion.scatter(np.random.rand(100000), np.random.rand(100000))"
