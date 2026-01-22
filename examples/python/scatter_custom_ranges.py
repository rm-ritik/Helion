"""
Custom Range Mapping Example

This example demonstrates how to use custom x_range and y_range parameters
to map data to different coordinate spaces instead of the default [-1, 1].
"""

import helion
import numpy as np

def main():
    print("Helion Custom Range Mapping Example")
    print("=" * 60)
    
    # Generate test data in [0, 100] range
    np.random.seed(42)
    n = 50_000
    x = np.random.uniform(0, 100, n).astype(np.float32)
    y = np.random.uniform(0, 100, n).astype(np.float32)
    
    print(f"\nGenerated {n:,} points")
    print(f"X data range: [{x.min():.2f}, {x.max():.2f}]")
    print(f"Y data range: [{y.min():.2f}, {y.max():.2f}]")
    
    # Example 1: Default mapping to [-1, 1] (GPU clip space)
    print("\n1. Default mapping to [-1, 1]:")
    plot1 = helion.scatter(x, y, color="#3B82F6")
    print("   ✓ Data normalized to GPU clip space [-1, 1]")
    
    # Example 2: Map to [0, 1] range (common for textures/normalized coords)
    print("\n2. Custom mapping to [0, 1]:")
    plot2 = helion.scatter(
        x, y, 
        color="#10B981",
        x_range=(0.0, 1.0),
        y_range=(0.0, 1.0)
    )
    print("   ✓ Data normalized to [0, 1] range")
    
    # Example 3: Inverted Y-axis (common in screen coordinates)
    print("\n3. Inverted Y-axis [1, -1]:")
    plot3 = helion.scatter(
        x, y,
        color="#F59E0B", 
        x_range=(-1.0, 1.0),
        y_range=(1.0, -1.0)  # Top to bottom
    )
    print("   ✓ Y-axis inverted (screen-space style)")
    
    # Example 4: Asymmetric ranges
    print("\n4. Asymmetric mapping [-0.5, 0.5] x [-1, 1]:")
    plot4 = helion.scatter(
        x, y,
        color="#EF4444",
        x_range=(-0.5, 0.5),
        y_range=(-1.0, 1.0)
    )
    print("   ✓ X compressed, Y at full range")
    
    # Example 5: Preserve aspect ratio by using data bounds
    print("\n5. Data-driven square mapping:")
    # Map to square [-1, 1] preserving data aspect ratio
    data_aspect = (x.max() - x.min()) / (y.max() - y.min())
    if data_aspect > 1:
        # Data is wider than tall
        x_out = (-1.0, 1.0)
        y_out = (-1.0/data_aspect, 1.0/data_aspect)
    else:
        # Data is taller than wide
        x_out = (-data_aspect, data_aspect)
        y_out = (-1.0, 1.0)
    
    plot5 = helion.scatter(
        x, y,
        color="#8B5CF6",
        x_range=x_out,
        y_range=y_out
    )
    print(f"   ✓ Aspect ratio preserved: {data_aspect:.2f}")
    print(f"   ✓ X mapped to [{x_out[0]:.2f}, {x_out[1]:.2f}]")
    print(f"   ✓ Y mapped to [{y_out[0]:.2f}, {y_out[1]:.2f}]")
    
    print("\n" + "=" * 60)
    print("Custom ranges allow precise control over coordinate mapping.")
    print("Use cases:")
    print("  • [-1, 1]: GPU clip space (default)")
    print("  • [0, 1]: Texture coordinates, normalized space")
    print("  • [1, -1]: Screen space (top-to-bottom)")
    print("  • Custom: Preserve aspect ratios, zoom to regions")
    print("\nOpening window with aspect-ratio preserved plot...")
    print("Close window to exit.")
    print("=" * 60)
    
    # Show the last plot (aspect ratio preserved)
    plot5.show()

if __name__ == "__main__":
    main()
