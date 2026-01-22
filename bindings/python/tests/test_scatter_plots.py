"""
Integration tests for Helion Python bindings

These tests verify that scatter plots can be created without errors
and that the data structures are correctly initialized.
"""

import pytest
import helion
import numpy as np


class TestScatterPlotCreation:
    """Test scatter plot creation with various configurations"""
    
    def test_basic_scatter_plot(self):
        """Test creating a basic scatter plot"""
        x = np.random.rand(1000)
        y = np.random.rand(1000)
        
        plot = helion.scatter(x, y)
        assert plot is not None
        assert isinstance(plot, helion.ScatterPlot)
    
    def test_scatter_with_color_hex(self):
        """Test scatter plot with hex color"""
        x = np.random.rand(1000)
        y = np.random.rand(1000)
        
        plot = helion.scatter(x, y, color="#FF5733")
        assert plot is not None
    
    def test_scatter_with_color_rgb(self):
        """Test scatter plot with RGB tuple"""
        x = np.random.rand(1000)
        y = np.random.rand(1000)
        
        plot = helion.scatter(x, y, color=(1.0, 0.5, 0.3))
        assert plot is not None
    
    def test_scatter_with_custom_size(self):
        """Test scatter plot with custom point size"""
        x = np.random.rand(1000)
        y = np.random.rand(1000)
        
        plot = helion.scatter(x, y, size=5.0)
        assert plot is not None
    
    def test_scatter_with_custom_dimensions(self):
        """Test scatter plot with custom viewport dimensions"""
        x = np.random.rand(1000)
        y = np.random.rand(1000)
        
        plot = helion.scatter(x, y, width=1200.0, height=800.0)
        assert plot is not None
    
    def test_scatter_with_custom_ranges(self):
        """Test scatter plot with custom coordinate ranges"""
        x = np.random.rand(1000)
        y = np.random.rand(1000)
        
        plot = helion.scatter(
            x, y,
            x_range=(0.0, 1.0),
            y_range=(0.0, 1.0)
        )
        assert plot is not None
    
    def test_large_dataset(self):
        """Test scatter plot with 1 million points"""
        x = np.random.rand(1_000_000)
        y = np.random.rand(1_000_000)
        
        plot = helion.scatter(x, y)
        assert plot is not None
    
    def test_float64_auto_conversion(self):
        """Test that float64 arrays are automatically converted"""
        x = np.random.rand(1000)  # float64 by default
        y = np.random.rand(1000)  # float64 by default
        
        # Should not raise an error
        plot = helion.scatter(x, y)
        assert plot is not None
    
    def test_mismatched_array_lengths(self):
        """Test handling of mismatched x/y array lengths"""
        x = np.random.rand(1000)
        y = np.random.rand(500)
        
        # Should create plot but use shorter length
        with pytest.warns(UserWarning, match="different lengths"):
            plot = helion.scatter(x, y)
        assert plot is not None


class TestColorClass:
    """Test the Color class functionality"""
    
    def test_color_from_hex(self):
        """Test creating color from hex string"""
        color = helion.Color.from_hex("#FF5733")
        assert 0.0 <= color.r <= 1.0
        assert 0.0 <= color.g <= 1.0
        assert 0.0 <= color.b <= 1.0
        assert color.a == 1.0
    
    def test_color_from_hex_with_alpha(self):
        """Test creating color from hex string with alpha"""
        color = helion.Color.from_hex("#FF5733AA")
        assert 0.0 <= color.r <= 1.0
        assert 0.0 <= color.g <= 1.0
        assert 0.0 <= color.b <= 1.0
        assert 0.0 <= color.a <= 1.0
    
    def test_color_constructor(self):
        """Test creating color with constructor"""
        color = helion.Color(1.0, 0.5, 0.3, 0.8)
        assert color.r == 1.0
        assert color.g == 0.5
        assert abs(color.b - 0.3) < 1e-6  # Float precision
        assert abs(color.a - 0.8) < 1e-6


class TestPoint2D:
    """Test the Point2D class functionality"""
    
    def test_point2d_constructor(self):
        """Test creating Point2D"""
        point = helion.Point2D(1.5, 2.5)
        assert point.x == 1.5
        assert point.y == 2.5


class TestPlotTitle:
    """Test plot title customization"""
    
    def test_set_title(self):
        """Test setting custom window title"""
        x = np.random.rand(100)
        y = np.random.rand(100)
        
        plot = helion.scatter(x, y)
        plot.set_title("My Custom Plot")
        # Should not raise an error
        assert plot is not None


class TestErrorHandling:
    """Test error handling"""
    
    def test_show_without_data(self):
        """Test that show() raises error when no data is set"""
        plot = helion.ScatterPlot()
        
        with pytest.raises(ValueError, match="No data set"):
            plot.show()
    
    def test_invalid_color_format(self):
        """Test that invalid color format raises error"""
        x = np.random.rand(100)
        y = np.random.rand(100)
        
        with pytest.raises(TypeError):
            plot = helion.scatter(x, y, color=12345)  # Invalid type


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
