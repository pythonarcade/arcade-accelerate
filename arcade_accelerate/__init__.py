import arcade
import arcade_accelerate_rust


def bootstrap():
    """Replace arcade math functions with rust accelerated versions."""
    patch_rotate_point()
    patch_hitboxes()


def patch_hitboxes():
    print("hereere")
    arcade.hitbox.base.HitBox = arcade_accelerate_rust.HitBox
    arcade.hitbox.base.AdjustableHitBox = arcade_accelerate_rust.AdjustableHitBox


def patch_rotate_point():
    arcade.math.rotate_point = arcade_accelerate_rust.rotate_point
