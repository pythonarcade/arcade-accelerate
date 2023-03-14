use pyo3::prelude::*;

#[pyfunction]
fn rotate_point(point: (f32, f32), center: (f32, f32), angle: f32) -> (f32, f32) {
    let (x, y) = point;
    let (cx, cy) = center;
    let s = angle.sin();
    let c = angle.cos();

    // translate point back to origin:
    let x = x - cx;
    let y = y - cy;

    // rotate point
    let xnew = x * c - y * s;
    let ynew = x * s + y * c;

    // translate point back:
    let x = xnew + cx;
    let y = ynew + cy;

    (x, y)
}

/// A Python module implemented in Rust.
#[pymodule]
fn arcade_accelerate_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rotate_point, m)?)?;
    Ok(())
}
