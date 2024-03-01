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


def patch_geometry(patches):
    patches["arcade.geometry"].are_polygons_intersecting = (
        arcade_accelerate.are_polygons_intersecting
    )
