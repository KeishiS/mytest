use std::slice;

#[cfg(feature = "python")]
use pyo3::prelude::*;

fn _func(a: i64, b: u32) -> i64 {
    a.pow(b)
}

fn _mat(mat_ptr: *const i64, rows: u32, cols: u32, result_ptr: *mut i64) {
    let data = unsafe { slice::from_raw_parts(mat_ptr, (rows * cols) as usize) };
    let result = unsafe { slice::from_raw_parts_mut(result_ptr, (rows * cols) as usize) };

    for i in 0..(rows * cols) as usize {
        result[i] = data[i] * 2;
    }
}

#[no_mangle]
#[cfg(feature = "julia")]
pub extern "C" fn jl_func(a: i64, b: u32) -> i64 {
    _func(a, b)
}

#[no_mangle]
#[cfg(feature = "julia")]
pub extern "C" fn jl_mat(mat_ptr: *const i64, rows: u32, cols: u32, result_ptr: *mut i64) {
    _mat(mat_ptr, rows, cols, result_ptr);
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
