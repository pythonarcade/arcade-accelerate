use pyo3::prelude::*;

mod hitbox;
pub use hitbox::{HitBox, RotatableHitBox};

mod math;
pub use math::{clamp, rotate_point};

mod geometry;
pub use geometry::are_polygons_intersecting;

/// A Python module implemented in Rust.
#[pymodule]
fn arcade_accelerate(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<HitBox>()?;
    m.add_class::<RotatableHitBox>()?;
    m.add_function(wrap_pyfunction!(rotate_point, m)?)?;
    m.add_function(wrap_pyfunction!(clamp, m)?)?;
    m.add_function(wrap_pyfunction!(are_polygons_intersecting, m)?)?;
    Ok(())
}
