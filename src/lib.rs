use pyo3::prelude::*;

mod hitbox;
pub use hitbox::*;

mod math;
pub use math::*;

mod geometry;
pub use geometry::*;

mod sprite_list;
pub use sprite_list::*;

/// A Python module implemented in Rust.
#[pymodule]
fn arcade_accelerate(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<hitbox::HitBox>()?;
    m.add_class::<hitbox::RotatableHitBox>()?;
    m.add_function(wrap_pyfunction!(math::rotate_point, m)?)?;
    m.add_function(wrap_pyfunction!(math::clamp, m)?)?;
    m.add_function(wrap_pyfunction!(math::lerp, m)?)?;
    m.add_function(wrap_pyfunction!(math::lerp_vec, m)?)?;
    m.add_function(wrap_pyfunction!(math::lerp_angle, m)?)?;
    m.add_function(wrap_pyfunction!(math::get_distance, m)?)?;
    m.add_function(wrap_pyfunction!(math::get_angle_degrees, m)?)?;
    m.add_function(wrap_pyfunction!(math::get_angle_radians, m)?)?;
    m.add_function(wrap_pyfunction!(math::rand_in_rect, m)?)?;
    m.add_function(wrap_pyfunction!(math::rand_in_circle, m)?)?;
    m.add_function(wrap_pyfunction!(math::rand_on_circle, m)?)?;
    m.add_function(wrap_pyfunction!(math::rand_on_line, m)?)?;
    m.add_function(wrap_pyfunction!(math::rand_angle_360_deg, m)?)?;
    m.add_function(wrap_pyfunction!(math::rand_angle_spread_deg, m)?)?;
    m.add_function(wrap_pyfunction!(math::rand_vec_degree_spread, m)?)?;
    m.add_function(wrap_pyfunction!(math::rand_vec_magnitude, m)?)?;
    m.add_function(wrap_pyfunction!(geometry::are_polygons_intersecting, m)?)?;
    m.add_function(wrap_pyfunction!(geometry::is_point_in_polygon, m)?)?;
    m.add_function(wrap_pyfunction!(geometry::is_point_in_box, m)?)?;
    m.add_function(wrap_pyfunction!(geometry::get_triangle_orientation, m)?)?;
    m.add_function(wrap_pyfunction!(geometry::are_lines_intersecting, m)?)?;
    m.add_function(wrap_pyfunction!(
        sprite_list::check_for_collision_with_list,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        sprite_list::check_for_collision_with_lists,
        m
    )?)?;

    Ok(())
}
