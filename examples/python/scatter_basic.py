"""
Basic Scatter Plot Example

This example demonstrates how to create a simple scatter plot
with 100,000 random data points using Helion.
"""

import helion
import numpy as np

def main():
    print("Helion Scatter Plot Example")
    print(f"Version: {helion.__version__}")
    print("-" * 50)
    
    # Generate random data
    np.random.seed(42)
    n_points = 100_000
    
    print(f"Generating {n_points:,} random points...")
    x = np.random.rand(n_points).astype(np.float32)
    y = np.random.rand(n_points).astype(np.float32)
    
    # Create scatter plot with custom color
    print("Creating scatter plot...")
    plot = helion.scatter(x, y, color="#FF5733")
    
    print("✓ Scatter plot created successfully!")
    print(f"  - Points: {n_points:,}")
    print(f"  - X range: [{x.min():.3f}, {x.max():.3f}]")
    print(f"  - Y range: [{y.min():.3f}, {y.max():.3f}]")
    print(f"  - Color: #FF5733 (coral red)")

if __name__ == "__main__":
    main()
