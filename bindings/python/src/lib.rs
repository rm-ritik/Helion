use pyo3::prelude::*;

/// Helion Python bindings
#[pymodule]
fn _helion(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    
    // Chart creation functions will be added here
    // m.add_function(wrap_pyfunction!(scatter, m)?)?;
    // m.add_function(wrap_pyfunction!(line, m)?)?;
    
    Ok(())
}
