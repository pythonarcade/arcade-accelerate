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

#[pyfunction]
pub fn lerp(v1: f32, v2: f32, u: f32) -> f32 {
    v1 + ((v2 - v1) * u)
}

#[pyfunction]
pub fn lerp_vec(v1: (f32, f32), v2: (f32, f32), u: f32) -> (f32, f32) {
    (lerp(v1.0, v2.0, u), lerp(v1.1, v2.1, u))
}

#[pyfunction]
pub fn lerp_angle(start_angle: f32, end_angle: f32, u: f32) -> f32 {
    
    let mut temp_start = start_angle % 360.0;
    let mut temp_end = end_angle % 360.0;

    while temp_start - temp_end > 180.0 {
        temp_end += 360.0;
    }

    while temp_start - temp_end < - 180.0 {
        temp_end -= 360.0;
    }

    lerp(temp_start, temp_end, u) % 360.0
}
