use pyo3::prelude::*;

#[pyfunction]
pub fn rotate_point(x: f32, y: f32, cx: f32, cy: f32, angle: f32) -> (f32, f32) {
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

#[pyfunction]
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
