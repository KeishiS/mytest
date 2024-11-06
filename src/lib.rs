#[cfg(feature = "python")]
use pyo3::prelude::*;

fn _func(a: i64, b: u32) -> i64 {
    a.pow(b)
}

#[no_mangle]
#[cfg(feature = "julia")]
pub extern "C" fn jl_func(a: i64, b: u32) -> i64 {
    _func(a, b)
}

#[cfg(feature = "python")]
#[pyfunction]
fn py_func(a: i64, b: u32) -> i64 {
    _func(a, b)
}

#[cfg(feature = "python")]
#[pymodule]
fn mytest(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_func, m)?)?;
    Ok(())
}
