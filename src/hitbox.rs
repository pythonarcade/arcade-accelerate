use pyo3::prelude::*;

#[pyclass(subclass, module = "arcade.hitbox.base")]
pub struct HitBox {
    #[pyo3(get, set)]
    points: Vec<(f32, f32)>,
    #[pyo3(get, set)]
    position: (f32, f32),
    #[pyo3(get, set)]
    scale: (f32, f32),
}

#[pymethods]
impl HitBox {
    #[new]
    fn new(
        points: Vec<(f32, f32)>,
        position: Option<(f32, f32)>,
        scale: Option<(f32, f32)>,
    ) -> HitBox {
        let final_position = position.unwrap_or((0.0, 0.0));
        let final_scale = scale.unwrap_or((1.0, 1.0));
        HitBox {
            points,
            position: final_position,
            scale: final_scale,
        }
    }

    fn create_rotatable(
        self_: PyRef<'_, Self>,
        py: Python<'_>,
        angle: Option<f32>,
    ) -> PyResult<Py<RotatableHitBox>> {
        let adjustable: Py<RotatableHitBox> = Py::new(
            py,
            RotatableHitBox::new(
                self_.points.to_vec(),
                Some(self_.position),
                Some(self_.scale),
                angle,
            ),
        )
        .unwrap();
        Ok(adjustable)
    }

    fn get_adjusted_points(self_: PyRef<'_, Self>) -> Vec<(f32, f32)> {
        let mut new_points: Vec<(f32, f32)> = Vec::with_capacity(self_.points.len());

        for point in self_.points.iter() {
            let x = (point.0 * self_.scale.0) + self_.position.0;
            let y = (point.1 * self_.scale.1) + self_.position.1;
            new_points.push((x, y));
        }

        new_points
    }

    #[getter]
    fn left(self_: PyRef<'_, Self>) -> PyResult<f32> {
        let mut converted = HitBox::get_adjusted_points(self_);
        converted.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        Ok(converted[0].0)
    }

    #[getter]
    fn right(self_: PyRef<'_, Self>) -> PyResult<f32> {
        let mut converted: Vec<(f32, f32)> = HitBox::get_adjusted_points(self_);
        converted.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        Ok(converted[0].0)
    }

    #[getter]
    fn bottom(self_: PyRef<'_, Self>) -> PyResult<f32> {
        let mut converted: Vec<(f32, f32)> = HitBox::get_adjusted_points(self_);
        converted.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        Ok(converted[0].1)
    }

    #[getter]
    fn top(self_: PyRef<'_, Self>) -> PyResult<f32> {
        let mut converted: Vec<(f32, f32)> = HitBox::get_adjusted_points(self_);
        converted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        Ok(converted[0].1)
    }
}

#[pyclass(extends=HitBox, module = "arcade.hitbox.base")]
pub struct RotatableHitBox {
    #[pyo3(get, set)]
    angle: f32,
}

#[pymethods]
impl RotatableHitBox {
    #[new]
    fn new(
        points: Vec<(f32, f32)>,
        position: Option<(f32, f32)>,
        scale: Option<(f32, f32)>,
        angle: Option<f32>,
    ) -> (Self, HitBox) {
        let final_angle = angle.unwrap_or(0.0);
        (
            RotatableHitBox { angle: final_angle },
            HitBox::new(points, position, scale),
        )
    }

    fn get_adjusted_points(self_: PyRef<'_, Self>) -> Vec<(f32, f32)> {
        let super_: &HitBox = self_.as_ref();
        let mut new_points: Vec<(f32, f32)> = Vec::with_capacity(super_.points.len());

        let rad = self_.angle.to_radians();
        let rad_cos = rad.cos();
        let rad_sin = rad.sin();
        for point in super_.points.iter() {
            let x = ((point.0 * rad_cos - point.1 * rad_sin) * super_.scale.0) + super_.position.0;
            let y = ((point.0 * rad_sin + point.1 * rad_cos) * super_.scale.1) + super_.position.1;
            new_points.push((x, y));
        }

        new_points
    }

    #[getter]
    fn left(self_: PyRef<'_, Self>) -> PyResult<f32> {
        let mut converted: Vec<(f32, f32)> = RotatableHitBox::get_adjusted_points(self_);
        converted.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        Ok(converted[0].0)
    }

    #[getter]
    fn right(self_: PyRef<'_, Self>) -> PyResult<f32> {
        let mut converted: Vec<(f32, f32)> = RotatableHitBox::get_adjusted_points(self_);
        converted.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        Ok(converted[0].0)
    }

    #[getter]
    fn bottom(self_: PyRef<'_, Self>) -> PyResult<f32> {
        let mut converted: Vec<(f32, f32)> = RotatableHitBox::get_adjusted_points(self_);
        converted.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        Ok(converted[0].1)
    }

    #[getter]
    fn top(self_: PyRef<'_, Self>) -> PyResult<f32> {
        let mut converted: Vec<(f32, f32)> = RotatableHitBox::get_adjusted_points(self_);
        converted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        Ok(converted[0].1)
    }
}
