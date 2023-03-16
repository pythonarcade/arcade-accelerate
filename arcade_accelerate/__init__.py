import sys

import arcade
import arcade_accelerate_rust


def bootstrap():
    """Replace arcade math functions with rust accelerated versions."""
    patch_rotate_point()
    patch_hitboxes()

    exclude = ["arcade.hitbox.base", "arcade.math"]

    pkgs = []
    for mod in exclude:
        pkg = mod.split('.', 1)[0]
        pkgs.append(pkg)

    to_uncache = []
    for mod in sys.modules:
        if mod in exclude:
            continue

        if mod in pkgs:
            to_uncache.append(mod)
            continue

        for pkg in pkgs:
            if mod.startswith(pkg + '.'):
                to_uncache.append(mod)
                break

    for mod in to_uncache:
        del sys.modules[mod]

def patch_hitboxes():
    arcade.hitbox.base.HitBox = arcade_accelerate_rust.HitBox
    arcade.hitbox.base.AdjustableHitBox = arcade_accelerate_rust.AdjustableHitBox


def patch_rotate_point():
    arcade.math.rotate_point = arcade_accelerate_rust.rotate_point
    arcade.math.rotate_point = arcade_accelerate_rust.rotate_point
    arcade.math.rotate_point = arcade_accelerate_rust.rotate_point
