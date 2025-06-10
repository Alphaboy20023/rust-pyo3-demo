use pyo3::prelude::*;

#[pyfunction]
fn reverse_string(s: String) -> String {
    s.chars().rev().collect()
}

#[pymodule]
fn rust_pyo3_demo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(reverse_string, m)?)?;
    Ok(())
}
