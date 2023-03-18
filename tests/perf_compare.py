"""
Not really a test module, but quick and dirty way to compare speed.
"""
import timeit


def print_results(name, r1, r2):
    ratio  = r1 / r2
    print(
        f"{name}: python={r1:.6f} rust={r2:.6f} -> {ratio:.3f} x",
        f"{'faster' if ratio > 1 else 'slower'}",
    )


def compare_functions(command_1: str, command_2: str, command_1_setup: str = "import arcade"):
    r1 = timeit.timeit(
        command_1,
        setup=command_1_setup,
        number=1_000_000,
    )
    r2 = timeit.timeit(
        f'arcade_accelerate.{command_2}',
        setup='from arcade_accelerate import arcade_accelerate',
        number=1_000_000,
    )
    print_results(command_2, r1, r2)


if __name__ == '__main__':
    compare_functions(
        command_1='arcade.math.rotate_point(2.0, 2.0, 3.0, 3.0, 90.0)',
        command_2='rotate_point(2.0, 2.0, 3.0, 3.0, 90)',
    )
    compare_functions(
        command_1="arcade.math.clamp(2.0, 2.0, 3.0)",
        command_2="clamp(2.0, 2.0, 3.0)",
    )
    compare_functions(
        command_1='arcade.geometry.geometry_python.are_polygons_intersecting(((100.0, 100.0),(100.0, 200.0),(200.0, 200.0),(200.0, 100.0)), ((125.0, 125.0),(125.0, 175.0),(175.0, 175.0),(175.0, 125.0)))',
        command_2='are_polygons_intersecting(((100.0, 100.0),(100.0, 200.0),(200.0, 200.0),(200.0, 100.0)), ((125.0, 125.0),(125.0, 175.0),(175.0, 175.0),(175.0, 125.0)))'
    )
