use crate::geometry::are_polygons_intersecting;
use crate::hitbox::HitBox;
use pyo3::prelude::*;

#[pyfunction]
pub fn check_for_collision_with_list(
    py: Python<'_>,
    sprite: &PyAny,
    sprite_list: PyObject,
) -> Vec<PyObject> {
    let mut final_sprites: Vec<PyObject> = Vec::new();

    let sprite_list_list = sprite_list.getattr(py, "sprite_list").unwrap();
    let sprites_to_check: Vec<PyObject> = sprite_list_list.extract(py).unwrap();

    for sprite2 in sprites_to_check.iter() {
        let mut hitbox1: HitBox = sprite.getattr("_hit_box").unwrap().extract().unwrap();
        let mut hitbox2: HitBox = sprite2
            .getattr(py, "_hit_box")
            .unwrap()
            .extract(py)
            .unwrap();

        let check_2 = are_polygons_intersecting(
            HitBox::get_adjusted_points_native(&mut hitbox1).to_vec(),
            HitBox::get_adjusted_points_native(&mut hitbox2).to_vec(),
        );

        if check_2 {
            let final_sprite = sprite2.to_object(py);
            final_sprites.push(final_sprite);
        }
    }

    final_sprites
}
