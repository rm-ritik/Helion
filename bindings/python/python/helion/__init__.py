"""
Helion - High-performance data visualization engine powered by WebGPU

Render millions of data points at 60 FPS using GPU acceleration.

Example:
    >>> import helion
    >>> import numpy as np
    >>> 
    >>> # Generate 100,000 random points
    >>> x = np.random.rand(100000)
    >>> y = np.random.rand(100000)
    >>> 
    >>> # Create scatter plot
    >>> plot = helion.scatter(x, y, color="#FF5733")
"""

from ._helion import (
    __version__,
    Point2D,
    Color,
    PyScatterPlot as ScatterPlot,
    scatter,
)

__all__ = [
    "__version__",
    "Point2D",
    "Color",
    "ScatterPlot",
    "scatter",
]
