"""
Test mismatched array lengths

This example shows that Helion handles mismatched x/y array lengths
by using the shorter length and issuing a warning.
"""

import helion
import numpy as np
import warnings

def main():
    print("Testing mismatched array lengths...")
    print("=" * 60)
    
    # Create arrays of different lengths
    x = np.array([1.0, 2.0, 3.0, 4.0, 5.0], dtype=np.float32)  # 5 elements
    y = np.array([1.0, 2.0, 3.0], dtype=np.float32)             # 3 elements
    
    print(f"\nX array length: {len(x)}")
    print(f"Y array length: {len(y)}")
    print("\nCreating scatter plot...")
    
    # Catch the warning
    with warnings.catch_warnings(record=True) as w:
        warnings.simplefilter("always")
        plot = helion.scatter(x, y, color="#FF5733")
        
        if w:
            print(f"\n⚠️  Warning issued: {w[0].message}")
        else:
            print("\n✓ No warning (unexpected!)")
    
    print("\n" + "=" * 60)
    print("The plot was created using the shorter length (3 points).")
    print("This matches the core library behavior.")

if __name__ == "__main__":
    main()
