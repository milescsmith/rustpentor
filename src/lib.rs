
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn above_below(x: f64, lower: f64, upper: f64) -> f64 {
    if x <= lower {
        lower
    } else if x >= upper {
        upper
    } else {
        x
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn milofuncs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(above_below, m)?)?;
    Ok(())
}