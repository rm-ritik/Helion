"""
Large Dataset Performance Example

This example demonstrates Helion's ability to handle very large datasets
efficiently by rendering 1 million points.
"""

import helion
import numpy as np
import time

def main():
    print("Helion Large Dataset Example")
    print(f"Version: {helion.__version__}")
    print("=" * 60)
    
    # Test with 1 million points
    n_points = 1_000_000
    
    print(f"\nGenerating {n_points:,} random points...")
    start_time = time.time()
    
    np.random.seed(42)
    x = np.random.randn(n_points).astype(np.float32)
    y = np.random.randn(n_points).astype(np.float32)
    
    gen_time = time.time() - start_time
    print(f"✓ Data generated in {gen_time:.3f}s")
    
    # Create scatter plot
    print("\nCreating scatter plot with GPU acceleration...")
    start_time = time.time()
    
    plot = helion.scatter(x, y, color="#3B82F6")
    
    plot_time = time.time() - start_time
    print(f"✓ Plot created in {plot_time:.3f}s")
    
    # Display statistics
    print("\n" + "=" * 60)
    print("RESULTS")
    print("=" * 60)
    print(f"Data Points:     {n_points:,}")
    print(f"X Range:         [{x.min():.3f}, {x.max():.3f}]")
    print(f"Y Range:         [{y.min():.3f}, {y.max():.3f}]")
    print(f"Memory (X+Y):    {(x.nbytes + y.nbytes) / 1024 / 1024:.2f} MB")
    print(f"Generation Time: {gen_time:.3f}s")
    print(f"Plot Time:       {plot_time:.3f}s")
    print(f"Total Time:      {gen_time + plot_time:.3f}s")
    print("=" * 60)
    
    print("\nThis demonstrates Helion's ability to handle large datasets")
    print("efficiently using WebGPU acceleration.")

if __name__ == "__main__":
    main()
