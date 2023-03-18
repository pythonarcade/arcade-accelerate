use pyo3::prelude::*;

static _PRECISION: u32 = 2;

#[pyfunction]
pub fn rotate_point(x: f32, y: f32, cx: f32, cy: f32, angle: f32) -> (f32, f32) {
    let angle_rads = angle.to_radians();
    let s = angle_rads.sin();
    let c = angle_rads.cos();

    // translate point back to origin:
    let temp_x = x - cx;
    let temp_y = y - cy;

    // rotate point
    let xnew = (temp_x * c - temp_y * s) + cx;
    let ynew = (temp_x * s + temp_y * c) + cy;

    let precision = 10i32.pow(_PRECISION) as f32;
    let x_rounded = (xnew * precision).round() / precision;
    let y_rounded = (ynew * precision).round() / precision;

    (x_rounded, y_rounded)
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
