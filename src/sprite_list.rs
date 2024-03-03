use crate::geometry::are_polygons_intersecting_native;
use crate::hitbox::{HitBox, NativeAdjustedPoints, RotatableHitBox};
use pyo3::intern;
use pyo3::prelude::*;

#[pyfunction]
pub fn check_for_collision_with_list(
    py: Python<'_>,
    sprite: &PyAny, //
    sprite_list: &PyAny,
    method: Option<i32>,
) -> Vec<PyObject> {
    let _final_method = method.unwrap_or(3);
    let mut final_sprites: Vec<PyObject> = Vec::new();

    let main_points: &Vec<(f32, f32)>;
    let mut hitbox1: HitBox;
    let mut hitbox2: RotatableHitBox;
    let hitbox_py_object: &PyAny = sprite.getattr(intern!(py, "_hit_box")).unwrap();

    if hitbox_py_object.is_instance_of::<HitBox>() {
        hitbox1 = hitbox_py_object.extract::<HitBox>().unwrap();
        main_points = hitbox1.get_adjusted_points_native();
    } else if hitbox_py_object.is_instance_of::<RotatableHitBox>() {
        hitbox2 = hitbox_py_object.extract::<RotatableHitBox>().unwrap();
        main_points = hitbox2.get_adjusted_points_native();
    } else {
        panic!("Unknown Hitbox Type")
    }

    let sprite_list_list = sprite_list.getattr(intern!(py, "sprite_list")).unwrap();
    let sprites_to_check: Vec<PyObject> = sprite_list_list.extract().unwrap();

    for sprite2 in sprites_to_check.iter() {
        let other_sprite: &PyAny = sprite2.as_ref(py);

        let other_points: &Vec<(f32, f32)>;
        let mut other_hitbox1: HitBox;
        let mut other_hitbox2: RotatableHitBox;
        let other_hitbox_py_object: &PyAny = other_sprite.getattr(intern!(py, "_hit_box")).unwrap();

        if other_hitbox_py_object.is_instance_of::<HitBox>() {
            other_hitbox1 = other_hitbox_py_object.extract::<HitBox>().unwrap();
            other_points = other_hitbox1.get_adjusted_points_native();
        } else if other_hitbox_py_object.is_instance_of::<RotatableHitBox>() {
            other_hitbox2 = other_hitbox_py_object.extract::<RotatableHitBox>().unwrap();
            other_points = other_hitbox2.get_adjusted_points_native();
        } else {
            panic!("Unknown Hitbox Type")
        }

        let check_2 = are_polygons_intersecting_native(main_points, other_points);

        if check_2 {
            final_sprites.push(sprite2.to_object(py));
        }
    }

    final_sprites
}

#[pyfunction]
pub fn check_for_collision_with_lists(
    py: Python<'_>,
    sprite: &PyAny, //
    sprite_lists: Vec<&PyAny>,
) -> Vec<PyObject> {
    let mut final_sprites: Vec<PyObject> = Vec::new();

    let main_points: &Vec<(f32, f32)>;
    let mut hitbox1: HitBox;
    let mut hitbox2: RotatableHitBox;
    let hitbox_py_object: &PyAny = sprite.getattr(intern!(py, "_hit_box")).unwrap();

    if hitbox_py_object.is_instance_of::<HitBox>() {
        hitbox1 = hitbox_py_object.extract::<HitBox>().unwrap();
        main_points = hitbox1.get_adjusted_points_native();
    } else if hitbox_py_object.is_instance_of::<RotatableHitBox>() {
        hitbox2 = hitbox_py_object.extract::<RotatableHitBox>().unwrap();
        main_points = hitbox2.get_adjusted_points_native();
    } else {
        panic!("Unknown Hitbox Type")
    }

    for sprite_list in sprite_lists.iter() {
        let sprite_list_list = sprite_list.getattr(intern!(py, "sprite_list")).unwrap();
        let sprites_to_check: Vec<PyObject> = sprite_list_list.extract().unwrap();

        for sprite2 in sprites_to_check.iter() {
            let other_sprite: &PyAny = sprite2.as_ref(py);

            let other_points: &Vec<(f32, f32)>;
            let mut other_hitbox1: HitBox;
            let mut other_hitbox2: RotatableHitBox;
            let other_hitbox_py_object: &PyAny =
                other_sprite.getattr(intern!(py, "_hit_box")).unwrap();

            if other_hitbox_py_object.is_instance_of::<HitBox>() {
                other_hitbox1 = other_hitbox_py_object.extract::<HitBox>().unwrap();
                other_points = other_hitbox1.get_adjusted_points_native();
            } else if other_hitbox_py_object.is_instance_of::<RotatableHitBox>() {
                other_hitbox2 = other_hitbox_py_object.extract::<RotatableHitBox>().unwrap();
                other_points = other_hitbox2.get_adjusted_points_native();
            } else {
                panic!("Unknown Hitbox Type")
            }

            let check_2 = are_polygons_intersecting_native(main_points, other_points);

            if check_2 {
                final_sprites.push(sprite2.to_object(py));
            }
        }
    }

    final_sprites
}
