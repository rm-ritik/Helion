use pyo3::prelude::*;
use numpy::PyReadonlyArray1;
use helion_core::{ChartData, GPUBackend, ScatterRenderer, Point2D, Color};

/// GPU-accelerated scatter plot renderer
#[pyclass]
pub struct PyScatterPlot {
    backend: Option<GPUBackend>,
    renderer: Option<ScatterRenderer>,
}

#[pymethods]
impl PyScatterPlot {
    #[new]
    fn new() -> Self {
        Self {
            backend: None,
            renderer: None,
        }
    }
    
    /// Create a scatter plot from numpy arrays
    /// 
    /// Args:
    ///     x: NumPy array of x coordinates
    ///     y: NumPy array of y coordinates
    ///     color: Optional tuple (r, g, b, a) with values 0.0-1.0. Default is blue.
    ///     size: Point size in pixels. Default is 2.0.
    ///     width: Viewport width in pixels. Default is 800.0.
    ///     height: Viewport height in pixels. Default is 600.0.
    ///     x_range: Optional tuple (min, max) for custom x-axis range
    ///     y_range: Optional tuple (min, max) for custom y-axis range
    /// 
    /// Returns:
    ///     Dictionary with plot information
    #[pyo3(signature = (x, y, color=None, size=None, width=800.0, height=600.0, x_range=None, y_range=None))]
    fn from_arrays(
        &mut self,
        py: Python,
        x: PyReadonlyArray1<f32>,
        y: PyReadonlyArray1<f32>,
        color: Option<(f32, f32, f32, f32)>,
        size: Option<f32>,
        width: f32,
        height: f32,
        x_range: Option<(f32, f32)>,
        y_range: Option<(f32, f32)>,
    ) -> PyResult<String> {
        let x_slice = x.as_slice()?;
        let y_slice = y.as_slice()?;
        
        // Warn if arrays have different lengths (core will use shorter length)
        if x_slice.len() != y_slice.len() {
            let min_len = x_slice.len().min(y_slice.len());
            py.import_bound("warnings")?
                .call_method1(
                    "warn",
                    (format!(
                        "x and y arrays have different lengths ({} vs {}). Using {} points.",
                        x_slice.len(), y_slice.len(), min_len
                    ),)
                )?;
        }
        
        // Create color
        let color_opt = color.map(|(r, g, b, a)| Color { r, g, b, a });
        
        // Create chart data with optional custom ranges
        let _chart_data = ChartData::from_scatter_with_range(
            x_slice,
            y_slice,
            color_opt,
            size,
            width,
            height,
            x_range,
            y_range,
        );
        
        Ok(format!(
            "Scatter plot created with {} points",
            x_slice.len()
        ))
    }
}

/// Create a scatter plot from Python lists or numpy arrays
/// 
/// Args:
///     x: List or NumPy array of x coordinates
///     y: List or NumPy array of y coordinates
///     color: Optional hex color string (e.g., "#FF5733") or RGB tuple
///     size: Point size in pixels. Default is 2.0.
///     width: Viewport width in pixels. Default is 800.0.
///     height: Viewport height in pixels. Default is 600.0.
///     x_range: Optional tuple (min, max) for custom x output range. Default is [-1.0, 1.0].
///     y_range: Optional tuple (min, max) for custom y output range. Default is [-1.0, 1.0].
/// 
/// Returns:
///     PyScatterPlot object
/// 
/// Example:
///     >>> import helion
///     >>> import numpy as np
///     >>> x = np.random.rand(100000)
///     >>> y = np.random.rand(100000)
///     >>> plot = helion.scatter(x, y, color="#FF5733")
///     >>> 
///     >>> # Custom range mapping to [0, 1] instead of [-1, 1]
///     >>> plot2 = helion.scatter(x, y, x_range=(0.0, 1.0), y_range=(0.0, 1.0))
#[pyfunction]
#[pyo3(signature = (x, y, color=None, size=None, width=800.0, height=600.0, x_range=None, y_range=None))]
fn scatter(
    py: Python,
    x: &Bound<'_, PyAny>,
    y: &Bound<'_, PyAny>,
    color: Option<&Bound<'_, PyAny>>,
    size: Option<f32>,
    width: f32,
    height: f32,
    x_range: Option<(f32, f32)>,
    y_range: Option<(f32, f32)>,
) -> PyResult<PyScatterPlot> {
    let mut plot = PyScatterPlot::new();
    
    // Convert inputs to float32 numpy arrays if they aren't already
    // NumPy defaults to float64, but GPUs work best with float32
    let np = py.import_bound("numpy")?;
    let x_array: PyReadonlyArray1<f32> = np
        .call_method1("asarray", (x, np.getattr("float32")?))?
        .extract()?;
    let y_array: PyReadonlyArray1<f32> = np
        .call_method1("asarray", (y, np.getattr("float32")?))?
        .extract()?;
    
    // Parse color if provided
    let color_tuple = if let Some(c) = color {
        if let Ok(hex) = c.extract::<String>() {
            let color = Color::from_hex(&hex);
            Some((color.r, color.g, color.b, color.a))
        } else if let Ok(rgba) = c.extract::<(f32, f32, f32, f32)>() {
            Some(rgba)
        } else if let Ok((r, g, b)) = c.extract::<(f32, f32, f32)>() {
            Some((r, g, b, 1.0))
        } else {
            return Err(pyo3::exceptions::PyTypeError::new_err(
                "color must be a hex string, (r, g, b) tuple, or (r, g, b, a) tuple"
            ));
        }
    } else {
        None
    };
    
    plot.from_arrays(py, x_array, y_array, color_tuple, size, width, height, x_range, y_range)?;
    Ok(plot)
}

/// Helion Python bindings
#[pymodule]
fn _helion(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    
    // Classes from core (with python feature enabled)
    m.add_class::<Point2D>()?;
    m.add_class::<Color>()?;
    m.add_class::<PyScatterPlot>()?;
    
    // Functions
    m.add_function(wrap_pyfunction!(scatter, m)?)?;
    
    Ok(())
}
