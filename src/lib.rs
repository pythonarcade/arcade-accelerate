use pyo3::prelude::*;
use pyo3::types::PyTuple;

#[pyclass(subclass)]
struct HitBox {
    #[pyo3(get)]
    points: Py<PyTuple>,
}

#[pymethods]
impl HitBox {
    #[new]
    fn __new__(points: Vec<(f32, f32)>) -> HitBox {
        Python::with_gil(|py| {
            let tuple: Py<PyTuple> = PyTuple::new(py, points).into();
            HitBox { points: tuple }
        })
    }

    fn create_adjustable(
        &self,
        py: Python<'_>,
        position: (f32, f32),
        angle: f32,
        scale: (f32, f32),
    ) -> PyResult<PyObject> {
        let v: Vec<(f32, f32)> = self.points.extract(py)?;
        let adjustable: PyObject =
            Py::new(py, AdjustableHitBox::__new__(v, position, angle, scale))
                .unwrap()
                .into_py(py);
        Ok(adjustable)
    }

    fn get_adjusted_points(&self) -> &Py<PyTuple> {
        &self.points
    }
}

#[pyclass(extends=HitBox)]
struct AdjustableHitBox {
    #[pyo3(get, set)]
    position: (f32, f32),
    #[pyo3(get, set)]
    angle: f32,
    #[pyo3(get, set)]
    scale: (f32, f32),
}

#[pymethods]
impl AdjustableHitBox {
    #[new]
    fn __new__(
        points: Vec<(f32, f32)>,
        position: (f32, f32),
        angle: f32,
        scale: (f32, f32),
    ) -> (Self, HitBox) {
        (
            AdjustableHitBox {
                position: position,
                angle: angle,
                scale: scale,
            },
            HitBox::__new__(points),
        )
    }

    fn get_adjusted_points(self_: PyRef<'_, Self>, py: Python<'_>) -> Vec<(f32, f32)> {
        let super_: &HitBox = self_.as_ref();
        let old_points: Vec<(f32, f32)> = super_
            .points
            .extract(py)
            .expect("Failed to convert PyTuple to Vec");
        let mut new_points: Vec<(f32, f32)> = Vec::with_capacity(old_points.len());

        let rad = self_.angle.to_radians();
        let rad_cos = rad.cos();
        let rad_sin = rad.sin();
        for point in old_points.iter() {
            new_points.push(self_.adjust_point(*point, rad_cos, rad_sin));
        }

        new_points
    }

    fn adjust_point(&self, point: (f32, f32), cos: f32, sin: f32) -> (f32, f32) {
        let (mut x, mut y) = point;
        x = ((x * cos - y * sin) * self.scale.0) + self.position.0;
        y = ((x * sin + y * cos) * self.scale.0) + self.position.0;
        (x, y)
    }

    #[getter]
    fn left(self_: PyRef<'_, Self>, py: Python<'_>) -> PyResult<f32> {
        let super_: &HitBox = self_.as_ref();
        let mut converted: Vec<(f32, f32)> = super_
            .points
            .extract(py)
            .expect("Failed to convert PyTuple to Vec");

        converted.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        Ok(converted[0].0)
    }

    #[getter]
    fn right(self_: PyRef<'_, Self>, py: Python<'_>) -> PyResult<f32> {
        let super_: &HitBox = self_.as_ref();
        let mut converted: Vec<(f32, f32)> = super_
            .points
            .extract(py)
            .expect("Failed to convert PyTuple to Vec");

        converted.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        Ok(converted[0].0)
    }

    #[getter]
    fn bottom(self_: PyRef<'_, Self>, py: Python<'_>) -> PyResult<f32> {
        let super_: &HitBox = self_.as_ref();
        let mut converted: Vec<(f32, f32)> = super_
            .points
            .extract(py)
            .expect("Failed to convert PyTuple to Vec");

        converted.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        Ok(converted[0].1)
    }

    #[getter]
    fn top(self_: PyRef<'_, Self>, py: Python<'_>) -> PyResult<f32> {
        let super_: &HitBox = self_.as_ref();
        let mut converted: Vec<(f32, f32)> = super_
            .points
            .extract(py)
            .expect("Failed to convert PyTuple to Vec");

        converted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        Ok(converted[0].1)
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
    m.add_class::<HitBox>()?;
    m.add_class::<AdjustableHitBox>()?;
    m.add_function(wrap_pyfunction!(rotate_point, m)?)?;
    m.add_function(wrap_pyfunction!(clamp, m)?)?;
    Ok(())
}
