import arcade
import arcade_accelerate_rust
 

def boostrap():
    """Replace arcade math functions with rust accelerated versions."""
    patch_rotate_point()


def patch_rotate_point():
    arcade.math.rotate_point = arcade_accelerate_rust.rotate_point
