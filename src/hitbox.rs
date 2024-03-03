use pyo3::prelude::*;

pub trait NativeAdjustedPoints {
    fn get_adjusted_points_native(&mut self) -> &Vec<(f32, f32)>;
}

#[derive(Clone)]
#[pyclass(module = "arcade.hitbox.base")]
pub struct HitBox {
    #[pyo3(get, set)]
    pub points: Vec<(f32, f32)>,
    #[pyo3(get)]
    pub position: (f32, f32),
    #[pyo3(get)]
    pub scale: (f32, f32),
    pub angle: f32,

    pub adjusted_cache: Vec<(f32, f32)>,
    pub cache_dirty: bool,
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
            angle: 0.0,
            adjusted_cache: vec![],
            cache_dirty: true,
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

    pub fn get_adjusted_points(&mut self) -> Vec<(f32, f32)> {
        if self.cache_dirty {
            self.adjusted_cache = Vec::with_capacity(self.points.len());
            for point in self.points.iter() {
                let x = (point.0 * self.scale.0) + self.position.0;
                let y = (point.1 * self.scale.1) + self.position.1;
                self.adjusted_cache.push((x, y));
            }
            self.cache_dirty = false;
        }

        self.adjusted_cache.to_vec()
    }

    #[setter]
    pub fn set_position(&mut self, value: (f32, f32)) -> PyResult<()> {
        self.position = value;
        self.cache_dirty = true;
        Ok(())
    }

    #[setter]
    pub fn set_scale(&mut self, value: (f32, f32)) -> PyResult<()> {
        self.scale = value;
        self.cache_dirty = true;
        Ok(())
    }

    #[getter]
    pub fn left(&mut self) -> PyResult<f32> {
        let mut converted: Vec<(f32, f32)> = self.get_adjusted_points_native().to_vec();
        converted.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        Ok(converted[0].0)
    }

    #[getter]
    pub fn right(&mut self) -> PyResult<f32> {
        let mut converted: Vec<(f32, f32)> = self.get_adjusted_points_native().to_vec();
        converted.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        Ok(converted[0].0)
    }

    #[getter]
    pub fn bottom(&mut self) -> PyResult<f32> {
        let mut converted: Vec<(f32, f32)> = self.get_adjusted_points_native().to_vec();
        converted.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        Ok(converted[0].1)
    }

    #[getter]
    pub fn top(&mut self) -> PyResult<f32> {
        let mut converted: Vec<(f32, f32)> = self.get_adjusted_points_native().to_vec();
        converted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        Ok(converted[0].1)
    }
}

impl NativeAdjustedPoints for HitBox {
    fn get_adjusted_points_native(&mut self) -> &Vec<(f32, f32)> {
        if self.cache_dirty {
            self.adjusted_cache = Vec::with_capacity(self.points.len());
            for point in self.points.iter() {
                let x = (point.0 * self.scale.0) + self.position.0;
                let y = (point.1 * self.scale.1) + self.position.1;
                self.adjusted_cache.push((x, y));
            }
            self.cache_dirty = false;
        }

        &self.adjusted_cache
    }
}

#[derive(Clone)]
#[pyclass(module = "arcade.hitbox.base")]
pub struct RotatableHitBox {
    #[pyo3(get, set)]
    pub points: Vec<(f32, f32)>,
    #[pyo3(get)]
    pub position: (f32, f32),
    #[pyo3(get)]
    pub scale: (f32, f32),
    #[pyo3(get)]
    pub angle: f32,

    pub adjusted_cache: Vec<(f32, f32)>,
    pub cache_dirty: bool,
}

#[pymethods]
impl RotatableHitBox {
    #[new]
    fn new(
        points: Vec<(f32, f32)>,
        position: Option<(f32, f32)>,
        scale: Option<(f32, f32)>,
        angle: Option<f32>,
    ) -> RotatableHitBox {
        let final_position = position.unwrap_or((0.0, 0.0));
        let final_scale = scale.unwrap_or((1.0, 1.0));
        let final_angle = angle.unwrap_or(0.0);
        RotatableHitBox {
            points,
            position: final_position,
            scale: final_scale,
            angle: final_angle,
            adjusted_cache: vec![],
            cache_dirty: true,
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

    pub fn get_adjusted_points(&mut self) -> Vec<(f32, f32)> {
        if self.cache_dirty {
            self.adjusted_cache = Vec::with_capacity(self.points.len());

            let rad = self.angle.to_radians();
            let rad_cos = rad.cos();
            let rad_sin = rad.sin();
            for point in self.points.iter() {
                let x = ((point.0 * rad_cos + point.1 * rad_sin) * self.scale.0) + self.position.0;
                let y = ((-point.0 * rad_sin + point.1 * rad_cos) * self.scale.1) + self.position.1;
                self.adjusted_cache.push((x, y));
            }
            self.cache_dirty = false;
        }

        self.adjusted_cache.to_vec()
    }

    #[setter]
    pub fn set_position(&mut self, value: (f32, f32)) -> PyResult<()> {
        self.position = value;
        self.cache_dirty = true;
        Ok(())
    }

    #[setter]
    pub fn set_scale(&mut self, value: (f32, f32)) -> PyResult<()> {
        self.scale = value;
        self.cache_dirty = true;
        Ok(())
    }

    #[setter]
    pub fn set_angle(&mut self, value: f32) -> PyResult<()> {
        self.angle = value;
        self.cache_dirty = true;
        Ok(())
    }

    #[getter]
    pub fn left(&mut self) -> PyResult<f32> {
        let mut converted: Vec<(f32, f32)> = self.get_adjusted_points_native().to_vec();
        converted.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        Ok(converted[0].0)
    }

    #[getter]
    pub fn right(&mut self) -> PyResult<f32> {
        let mut converted: Vec<(f32, f32)> = self.get_adjusted_points_native().to_vec();
        converted.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        Ok(converted[0].0)
    }

    #[getter]
    pub fn bottom(&mut self) -> PyResult<f32> {
        let mut converted: Vec<(f32, f32)> = self.get_adjusted_points_native().to_vec();
        converted.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        Ok(converted[0].1)
    }

    #[getter]
    pub fn top(&mut self) -> PyResult<f32> {
        let mut converted: Vec<(f32, f32)> = self.get_adjusted_points_native().to_vec();
        converted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        Ok(converted[0].1)
    }
}

impl NativeAdjustedPoints for RotatableHitBox {
    fn get_adjusted_points_native(&mut self) -> &Vec<(f32, f32)> {
        if self.cache_dirty {
            self.adjusted_cache = Vec::with_capacity(self.points.len());

            let rad = self.angle.to_radians();
            let rad_cos = rad.cos();
            let rad_sin = rad.sin();
            for point in self.points.iter() {
                let x = ((point.0 * rad_cos + point.1 * rad_sin) * self.scale.0) + self.position.0;
                let y = ((-point.0 * rad_sin + point.1 * rad_cos) * self.scale.1) + self.position.1;
                self.adjusted_cache.push((x, y));
            }
            self.cache_dirty = false;
        }

        &self.adjusted_cache
    }
}

// #[derive(Clone)]
// #[pyclass(extends=HitBox, module = "arcade.hitbox.base")]
// pub struct RotatableHitBox {
//     #[pyo3(get, set)]
//     angle: f32,
// }

// #[pymethods]
// impl RotatableHitBox {
//     #[new]
//     fn new(
//         points: Vec<(f32, f32)>,
//         position: Option<(f32, f32)>,
//         scale: Option<(f32, f32)>,
//         angle: Option<f32>,
//     ) -> (Self, HitBox) {
//         let final_angle = angle.unwrap_or(0.0);
//         (
//             RotatableHitBox { angle: final_angle },
//             HitBox::new(points, position, scale),
//         )
//     }

//     pub fn get_adjusted_points(self_: PyRef<'_, Self>) -> Vec<(f32, f32)> {
//         let super_: &HitBox = self_.as_ref();
//         let mut new_points: Vec<(f32, f32)> = Vec::with_capacity(super_.points.len());

//         let rad = self_.angle.to_radians();
//         let rad_cos = rad.cos();
//         let rad_sin = rad.sin();
//         for point in super_.points.iter() {
//             let x = ((point.0 * rad_cos + point.1 * rad_sin) * super_.scale.0) + super_.position.0;
//             let y = ((-point.0 * rad_sin + point.1 * rad_cos) * super_.scale.1) + super_.position.1;
//             new_points.push((x, y));
//         }

//         new_points
//     }

//     #[getter]
//     fn left(self_: PyRef<'_, Self>) -> PyResult<f32> {
//         let mut converted: Vec<(f32, f32)> = RotatableHitBox::get_adjusted_points(self_);
//         converted.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
//         Ok(converted[0].0)
//     }

//     #[getter]
//     fn right(self_: PyRef<'_, Self>) -> PyResult<f32> {
//         let mut converted: Vec<(f32, f32)> = RotatableHitBox::get_adjusted_points(self_);
//         converted.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
//         Ok(converted[0].0)
//     }

//     #[getter]
//     fn bottom(self_: PyRef<'_, Self>) -> PyResult<f32> {
//         let mut converted: Vec<(f32, f32)> = RotatableHitBox::get_adjusted_points(self_);
//         converted.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
//         Ok(converted[0].1)
//     }

//     #[getter]
//     fn top(self_: PyRef<'_, Self>) -> PyResult<f32> {
//         let mut converted: Vec<(f32, f32)> = RotatableHitBox::get_adjusted_points(self_);
//         converted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
//         Ok(converted[0].1)
//     }
// }

// impl RotatableHitBox {
//     pub fn get_adjusted_points_native(self) -> Vec<(f32, f32)> {
//         let mut new_points: Vec<(f32, f32)> = Vec::with_capacity(self.parent.points.len());

//         let rad = self.angle.to_radians();
//         let rad_cos = rad.cos();
//         let rad_sin = rad.sin();
//         for point in self.parent.points.iter() {
//             let x = ((point.0 * rad_cos - point.1 * rad_sin) * self.parent.scale.0)
//                 + self.parent.position.0;
//             let y = ((point.0 * rad_sin + point.1 * rad_cos) * self.parent.scale.1)
//                 + self.parent.position.1;
//             new_points.push((x, y));
//         }

//         new_points
//     }
// }
