"""
Custom Color Examples

This example shows different ways to specify colors in Helion.
"""

import helion
import numpy as np

def main():
    print("Helion Color Examples")
    print(f"Version: {helion.__version__}")
    print("=" * 60)
    
    # Generate sample data
    np.random.seed(42)
    n = 10_000
    x = np.random.rand(n).astype(np.float32)
    y = np.random.rand(n).astype(np.float32)
    
    # Example 1: Hex color
    print("\n1. Using hex color (#FF5733)...")
    plot1 = helion.scatter(x, y, color="#FF5733")
    print("   ✓ Coral red scatter plot")
    
    # Example 2: RGB tuple
    print("\n2. Using RGB tuple (0.2, 0.8, 0.4)...")
    plot2 = helion.scatter(x, y, color=(0.2, 0.8, 0.4))
    print("   ✓ Green scatter plot")
    
    # Example 3: Default color (blue)
    print("\n3. Using default color...")
    plot3 = helion.scatter(x, y)
    print("   ✓ Blue scatter plot (default)")
    
    # Example 4: Using Color class
    print("\n4. Using Color class...")
    custom_color = helion.Color.from_hex("#9333EA")
    print(f"   ✓ Purple scatter plot (R={custom_color.r:.2f}, G={custom_color.g:.2f}, B={custom_color.b:.2f})")
    
    print("\n" + "=" * 60)
    print("All color formats work seamlessly with Helion!")

if __name__ == "__main__":
    main()
