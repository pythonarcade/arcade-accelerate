use pyo3::prelude::*;

#[pyclass(subclass)]
pub struct HitBox {
    #[pyo3(get)]
    points: Vec<(f32, f32)>,
}

#[pymethods]
impl HitBox {
    #[new]
    fn new(points: Vec<(f32, f32)>) -> HitBox {
        HitBox { points: points }
    }

    fn create_adjustable(
        self_: PyRef<'_, Self>,
        py: Python<'_>,
        position: (f32, f32),
        angle: f32,
        scale: (f32, f32),
    ) -> PyResult<Py<AdjustableHitBox>> {
        let adjustable: Py<AdjustableHitBox> = Py::new(
            py,
            AdjustableHitBox::new(self_.points.to_vec(), position, angle, scale),
        )
        .unwrap();
        Ok(adjustable)
    }

    fn get_adjusted_points(&self) -> Vec<(f32, f32)> {
        self.points.to_vec()
    }
}

#[pyclass(extends=HitBox)]
pub struct AdjustableHitBox {
    position: (f32, f32),
    angle: f32,
    scale: (f32, f32),
}

#[pymethods]
impl AdjustableHitBox {
    #[new]
    fn new(
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
            HitBox::new(points),
        )
    }

    fn get_adjusted_points(self_: PyRef<'_, Self>) -> Vec<(f32, f32)> {
        let super_: &HitBox = self_.as_ref();
        let mut new_points: Vec<(f32, f32)> = Vec::with_capacity(super_.points.len());

        let rad = self_.angle.to_radians();
        let rad_cos = rad.cos();
        let rad_sin = rad.sin();
        for point in super_.points.iter() {
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
    fn left(self_: PyRef<'_, Self>) -> PyResult<f32> {
        let super_: &HitBox = self_.as_ref();
        let mut converted: Vec<(f32, f32)> = super_.points.to_vec();

        converted.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        Ok(converted[0].0)
    }

    #[getter]
    fn right(self_: PyRef<'_, Self>) -> PyResult<f32> {
        let super_: &HitBox = self_.as_ref();
        let mut converted: Vec<(f32, f32)> = super_.points.to_vec();

        converted.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        Ok(converted[0].0)
    }

    #[getter]
    fn bottom(self_: PyRef<'_, Self>) -> PyResult<f32> {
        let super_: &HitBox = self_.as_ref();
        let mut converted: Vec<(f32, f32)> = super_.points.to_vec();

        converted.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        Ok(converted[0].1)
    }

    #[getter]
    fn top(self_: PyRef<'_, Self>) -> PyResult<f32> {
        let super_: &HitBox = self_.as_ref();
        let mut converted: Vec<(f32, f32)> = super_.points.to_vec();

        converted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        Ok(converted[0].1)
    }
}
