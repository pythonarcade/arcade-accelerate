import sys

import arcade
from arcade_accelerate import arcade_accelerate  # type: ignore


def bootstrap():
    """Replace arcade math functions with rust accelerated versions."""
    patch_math()
    patch_geometry()
    patch_hitboxes()
    patch_spritelist_collision()
    patch_sprite()

    exclude = [
        "arcade.hitbox.base",
        "arcade.math",
        "arcade.geometry",
        "arcade.sprite_list.collision",
        "arcade.sprite.base",
    ]

    pkgs = []
    for mod in exclude:
        pkg = mod.split(".", 1)[0]
        pkgs.append(pkg)

    to_uncache = []
    for mod in sys.modules:
        if mod in exclude:
            continue

        if mod in pkgs:
            to_uncache.append(mod)
            continue

        for pkg in pkgs:
            if mod.startswith(pkg + "."):
                to_uncache.append(mod)
                break

    for mod in to_uncache:
        del sys.modules[mod]


def patch_hitboxes():
    arcade.hitbox.base.HitBox = arcade_accelerate.HitBox
    arcade.hitbox.base.RotatableHitBox = arcade_accelerate.RotatableHitBox


def patch_spritelist_collision():
    arcade.sprite_list.collision.check_for_collision_with_list = (
        arcade_accelerate.check_for_collision_with_list
    )
    arcade.sprite_list.collision.check_for_collision_with_lists = (
        arcade_accelerate.check_for_collision_with_lists
    )


def patch_math():
    arcade.math.rotate_point = arcade_accelerate.rotate_point


def patch_geometry():
    arcade.geometry.are_polygons_intersecting = (
        arcade_accelerate.are_polygons_intersecting
    )


def patch_sprite():
    import arcade.sprite.base

    arcade.sprite.base.BasicSprite = arcade_accelerate.BasicSprite
