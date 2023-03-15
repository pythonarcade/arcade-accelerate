use pyo3::prelude::*;

struct Point {
    x: f32,
    y: f32,
}

trait BaseHitBox {}

#[pyclass]
struct HitBox {
    points: Vec<(f32, f32)>,
}

#[pymethods]
impl HitBox {
    #[new]
    fn py_new(points: Vec<(f32, f32)>) -> Self {
        HitBox { points: points }
    }

    fn create_adjustable(
        &self,
        position: (f32, f32),
        angle: f32,
        scale: (f32, f32),
    ) -> AdjustableHitBox {
        let adjustable: AdjustableHitBox = AdjustableHitBox {
            hitbox: *self,
            position: position,
            angle: angle,
            scale: scale,
        };
        adjustable
    }

    fn get_adjusted_points(&self) -> &Vec<(f32, f32)> {
        &self.points
    }
}

#[pyclass]
struct AdjustableHitBox {
    hitbox: HitBox,
    position: (f32, f32),
    angle: f32,
    scale: (f32, f32),
}

#[pymethods]
impl AdjustableHitBox {
    fn get_adjusted_points(&self) -> Vec<(f32, f32)> {
        let mut new_points: Vec<(f32, f32)> = Vec::with_capacity(self.hitbox.points.len());

        let rad = &self.angle.to_radians();
        let rad_cos = rad.cos();
        let rad_sin = rad.sin();
        for point in self.hitbox.points.iter() {
            new_points.push(self.adjust_point(*point, rad_cos, rad_sin));
        }
        new_points
    }

    fn adjust_point(&self, point: (f32, f32), cos: f32, sin: f32) -> (f32, f32) {
        let (mut x, mut y) = point;
        x = ((x * cos - y * sin) * self.scale.0) + self.position.0;
        y = ((x * sin + y * cos) * self.scale.0) + self.position.0;
        (x, y)
    }
}

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

#[pyfunction]
fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn arcade_accelerate_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rotate_point, m)?)?;
    m.add_function(wrap_pyfunction!(clamp, m)?)?;
    Ok(())
}
