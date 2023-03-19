use crate::geometry::are_polygons_intersecting;
use crate::hitbox::HitBox;
use pyo3::prelude::*;
use pyo3::types::{PyFloat, PyList, PyTuple};

#[pyfunction]
pub fn check_for_collision(sprite1: &PyAny, sprite2: &PyAny) -> bool {
    let sprite1_pos: (f32, f32) = sprite1
        .getattr("_position")
        .unwrap()
        .downcast::<PyTuple>()
        .unwrap()
        .extract()
        .unwrap();
    let sprite1_width_any = sprite1.getattr("_width").unwrap();
    let sprite1_width: f32 = sprite1_width_any
        .downcast::<PyFloat>()
        .unwrap()
        .extract()
        .unwrap();
    let sprite1_height_any = sprite1.getattr("_height").unwrap();
    let sprite1_height: f32 = sprite1_height_any
        .downcast::<PyFloat>()
        .unwrap()
        .extract()
        .unwrap();
    let sprite2_pos: (f32, f32) = sprite2
        .getattr("_position")
        .unwrap()
        .downcast::<PyTuple>()
        .unwrap()
        .extract()
        .unwrap();
    let sprite2_width_any = sprite2.getattr("_width").unwrap();
    let sprite2_width: f32 = sprite2_width_any
        .downcast::<PyFloat>()
        .unwrap()
        .extract()
        .unwrap();
    let sprite2_height_any = sprite2.getattr("_height").unwrap();
    let sprite2_height: f32 = sprite2_height_any
        .downcast::<PyFloat>()
        .unwrap()
        .extract()
        .unwrap();

    let rad_sum_1: f32;
    if sprite1_width > sprite1_height {
        rad_sum_1 = sprite1_width
    } else {
        rad_sum_1 = sprite1_height
    }

    let rad_sum_2: f32;
    if sprite2_width > sprite2_height {
        rad_sum_2 = sprite2_width
    } else {
        rad_sum_2 = sprite2_height
    }

    let rad_sum = (rad_sum_1 + rad_sum_2) * 0.71;
    let rad_sum_x2 = rad_sum * rad_sum;

    let diff_x = sprite1_pos.0 - sprite2_pos.0;
    let diff_x2 = diff_x * diff_x;
    if diff_x2 > rad_sum_x2 {
        return false;
    }

    let diff_y = sprite1_pos.1 - sprite2_pos.1;
    let diff_y2 = diff_y * diff_y;
    if diff_y2 > rad_sum_x2 {
        return false;
    }

    let distance = diff_x2 + diff_y2;
    if distance > rad_sum_x2 {
        return false;
    }

    let hitbox1: HitBox = sprite1.getattr("_hit_box").unwrap().extract().unwrap();
    let hitbox2: HitBox = sprite2.getattr("_hit_box").unwrap().extract().unwrap();

    are_polygons_intersecting(HitBox::get_adjusted_points_native(&hitbox1), HitBox::get_adjusted_points_native(&hitbox2))
}

#[pyfunction]
pub fn check_for_collision_with_list(
    py: Python<'_>,
    sprite: &PyAny,
    sprite_list: PyObject,
) -> Vec<PyObject> {
    let mut final_sprites: Vec<PyObject> = Vec::new();

    let sprite_list_list = sprite_list.getattr(py, "sprite_list").unwrap();
    let sprites_to_check: &PyList = sprite_list_list.downcast::<PyList>(py).unwrap();

    for sprite2 in sprites_to_check.iter() {
        if check_for_collision(sprite, sprite2) {
            let final_sprite = sprite2.to_object(py);
            final_sprites.push(final_sprite)
        }
    }

    final_sprites
}
