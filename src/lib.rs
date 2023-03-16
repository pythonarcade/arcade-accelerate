use pyo3::prelude::*;

mod hitbox;
pub use hitbox::{AdjustableHitBox, HitBox};

mod math;
pub use math::{clamp, rotate_point};

/// A Python module implemented in Rust.
#[pymodule]
fn arcade_accelerate_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<HitBox>()?;
    m.add_class::<AdjustableHitBox>()?;
    m.add_function(wrap_pyfunction!(rotate_point, m)?)?;
    m.add_function(wrap_pyfunction!(clamp, m)?)?;
    Ok(())
}
