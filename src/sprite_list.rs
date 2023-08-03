use crate::geometry::are_polygons_intersecting;
use crate::hitbox::{HitBox, RotatableHitBox};
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

    let mut hitbox1: Option<HitBox> = None;
    let mut hitbox2: Option<PyRef<'_, RotatableHitBox>> = None;

    let cls: &str = sprite
        .getattr("_hit_box")
        .unwrap()
        .get_type()
        .name()
        .unwrap();

    match cls {
        "HitBox" => {
            hitbox1 = sprite.getattr("_hit_box").unwrap().extract().unwrap();
        }
        "RotatableHitBox" => {
            hitbox2 = sprite.getattr("_hit_box").unwrap().extract().unwrap();
        }
        _ => panic!(),
    }

    let main_points: Vec<(f32, f32)> = if let Some(value) = hitbox1 {
        value.get_adjusted_points_native()
    } else if let Some(value) = hitbox2 {
        RotatableHitBox::get_adjusted_points(value)
    } else {
        panic!("unknown hitbox type");
    };

    let sprite_list_list = sprite_list.getattr("sprite_list").unwrap();
    let sprites_to_check: Vec<PyObject> = sprite_list_list.extract().unwrap();

    for sprite2 in sprites_to_check.iter() {
        let other_sprite: &PyAny = sprite2.as_ref(py);
        let mut other_hitbox1: Option<HitBox> = None;
        let mut other_hitbox2: Option<PyRef<'_, RotatableHitBox>> = None;
        let other_cls: &str = other_sprite
            .getattr("_hit_box")
            .unwrap()
            .get_type()
            .name()
            .unwrap();

        match other_cls {
            "HitBox" => {
                other_hitbox1 = other_sprite.getattr("_hit_box").unwrap().extract().unwrap();
            }
            "RotatableHitBox" => {
                other_hitbox2 = other_sprite.getattr("_hit_box").unwrap().extract().unwrap();
            }
            _ => panic!(),
        }

        let other_points: Vec<(f32, f32)> = if let Some(value) = other_hitbox1 {
            value.get_adjusted_points_native()
        } else if let Some(value) = other_hitbox2 {
            RotatableHitBox::get_adjusted_points(value)
        } else {
            panic!("unknown hitbox type");
        };

        let check_2 = are_polygons_intersecting(main_points.to_vec(), other_points);

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
    let mut hitbox1: Option<HitBox> = None;
    let mut hitbox2: Option<PyRef<'_, RotatableHitBox>> = None;

    let cls: &str = sprite
        .getattr("_hit_box")
        .unwrap()
        .get_type()
        .name()
        .unwrap();

    match cls {
        "HitBox" => {
            hitbox1 = sprite.getattr("_hit_box").unwrap().extract().unwrap();
        }
        "RotatableHitBox" => {
            hitbox2 = sprite.getattr("_hit_box").unwrap().extract().unwrap();
        }
        _ => panic!(),
    }

    let main_points: Vec<(f32, f32)> = if let Some(value) = hitbox1 {
        value.get_adjusted_points_native()
    } else if let Some(value) = hitbox2 {
        RotatableHitBox::get_adjusted_points(value)
    } else {
        panic!("unknown hitbox type")
    };

    for sprite_list in sprite_lists.iter() {
        let sprite_list_list = sprite_list.getattr("sprite_list").unwrap();
        let sprites_to_check: Vec<PyObject> = sprite_list_list.extract().unwrap();

        for sprite2 in sprites_to_check.iter() {
            let other_sprite: &PyAny = sprite2.as_ref(py);
            let mut other_hitbox1: Option<HitBox> = None;
            let mut other_hitbox2: Option<PyRef<'_, RotatableHitBox>> = None;
            let other_cls: &str = other_sprite
                .getattr("_hit_box")
                .unwrap()
                .get_type()
                .name()
                .unwrap();
            match other_cls {
                "HitBox" => {
                    other_hitbox1 = other_sprite.getattr("_hit_box").unwrap().extract().unwrap();
                }
                "RotatableHitBox" => {
                    other_hitbox2 = other_sprite.getattr("_hit_box").unwrap().extract().unwrap();
                }
                _ => panic!(),
            }

            let other_points: Vec<(f32, f32)> = if let Some(value) = other_hitbox1 {
                value.get_adjusted_points_native()
            } else if let Some(value) = other_hitbox2 {
                RotatableHitBox::get_adjusted_points(value)
            } else {
                panic!("unknown hitbox type");
            };

            let check_2 = are_polygons_intersecting(main_points.to_vec(), other_points);

            if check_2 {
                final_sprites.push(sprite2.to_object(py));
            }
        }
    }

    final_sprites
}
