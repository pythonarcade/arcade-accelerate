"""
Not really a test module, but quick and dirty way to compare speed.
"""
import timeit


def print_results(name, r1, r2):
    ratio  = r2 / r1
    print(
        f"{name}: arcade={r1:.6f} rust={r2:.6f} {ratio:.6f} x",
        f"{'faster' if ratio > 1 else 'slower'}",
    )

def rotate_point():
    # rotate
    r1 = timeit.timeit(
        'arcade.math.rotate_point(2.0, 2.0, 3.0, 3.0, 90)',
        setup='import arcade',
        number=1_000_000,
    )
    r2 = timeit.timeit(
        'arcade_accelerate_rust.rotate_point((2.0, 2.0), (3.0, 3.0), 90)',
        setup='import arcade_accelerate_rust',
        number=1_000_000,
    )
    print_results('rotate_point', r1, r2)


if __name__ == '__main__':
    rotate_point()
