import sys

from arcade_accelerate import arcade_accelerate
from arcade_accelerate.module_patcher import (
    AutoPopulatingDictionary,
    PatchingMetaPathFinder,
)


def bootstrap():
    patches = AutoPopulatingDictionary()
    sys.meta_path.insert(0, PatchingMetaPathFinder(patches))

    """Replace arcade math functions with rust accelerated versions."""
    patch_math(patches)
    patch_geometry(patches)
    patch_hitboxes(patches)
    patch_spritelist_collision(patches)


def patch_hitboxes(patches):
    patches["arcade.hitbox.base"].HitBox = arcade_accelerate.HitBox
    patches["arcade.hitbox.base"].RotatableHitBox = arcade_accelerate.RotatableHitBox


def patch_spritelist_collision(patches):
    patches["arcade.sprite_list.collision"].check_for_collision_with_list = (
        arcade_accelerate.check_for_collision_with_list
    )
    patches["arcade.sprite_list.collision"].check_for_collision_with_lists = (
        arcade_accelerate.check_for_collision_with_lists
    )


def patch_math(patches):
    patches["arcade.math"].rotate_point = arcade_accelerate.rotate_point
    patches["arcade.math"].clamp = arcade_accelerate.clamp
    patches["arcade.math"].lerp = arcade_accelerate.lerp
    patches["arcade.math"].lerp_2d = arcade_accelerate.lerp_2d
    patches["arcade.math"].lerp_3d = arcade_accelerate.lerp_3d
    patches["arcade.math"].lerp_angle = arcade_accelerate.lerp_angle
    patches["arcade.math"].get_distance = arcade_accelerate.get_distance
    patches["arcade.math"].get_angle_degrees = arcade_accelerate.get_angle_degrees
    patches["arcade.math"].get_angle_radians = arcade_accelerate.get_angle_radians
    patches["arcade.math"].rand_in_rect = arcade_accelerate.rand_in_rect
    patches["arcade.math"].rand_in_circle = arcade_accelerate.rand_in_circle
    patches["arcade.math"].rand_on_circle = arcade_accelerate.rand_on_circle
    patches["arcade.math"].rand_on_line = arcade_accelerate.rand_on_line
    patches["arcade.math"].rand_angle_360_deg = arcade_accelerate.rand_angle_360_deg
    patches["arcade.math"].rand_angle_spread_deg = (
        arcade_accelerate.rand_angle_spread_deg
    )
    patches["arcade.math"].rand_vec_degree_spread = (
        arcade_accelerate.rand_vec_degree_spread
    )
    patches["arcade.math"].rand_vec_magnitude = arcade_accelerate.rand_vec_magnitude
    patches["arcade.math"].quaternion_rotation = arcade_accelerate.quaternion_rotation


def patch_geometry(patches):
    patches["arcade.geometry"].are_polygons_intersecting = (
        arcade_accelerate.are_polygons_intersecting
    )
    patches["arcade.geometry"].is_point_in_box = arcade_accelerate.is_point_in_box
    patches["arcade.geometry"].get_triangle_orientation = (
        arcade_accelerate.get_triangle_orientation
    )
    patches["arcade.geometry"].are_lines_intersecting = (
        arcade_accelerate.are_lines_intersecting
    )
    patches["arcade.geometry"].is_point_in_polygon = (
        arcade_accelerate.is_point_in_polygon
    )
