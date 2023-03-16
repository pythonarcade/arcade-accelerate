import arcade

from arcade_accelerate import arcade_accelerate


def bootstrap():
    """Replace arcade math functions with rust accelerated versions."""
    patch_rotate_point()
    patch_hitboxes()


def patch_hitboxes():
    arcade.hitbox.base.HitBox = arcade_accelerate.HitBox
    arcade.hitbox.base.AdjustableHitBox = arcade_accelerate.AdjustableHitBox


def patch_rotate_point():
    arcade.math.rotate_point = arcade_accelerate.rotate_point
    arcade.math.rotate_point = arcade_accelerate.rotate_point
