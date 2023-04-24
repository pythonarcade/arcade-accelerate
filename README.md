
# arcade-accelerate

An experimental library for accelerating [Arcade](https://github.com/pythonarcade/arcade) using Rust. The module can
be imported and monkey patch Arcade, replacing functions and types with rust versions.

```py
import arcade_accelerate
arcade_accelerate.bootstrap()

import arcade
```

It is important to run the arcade-accelerate bootstrapping process before importing Arcade, otherwise the monkey-patched versions will not be fully applied.

## Build / Setup

First create and activate a Python virtual environment, then install maturin:

```bash
pip install maturin
```

Install the crate as module in the current virtual environment using Maturin. Generally
when working on performance enhancements you will want to use the `--release` flag.

```sh
# Debug
maturin develop

# Release
maturin develop --release
```

Then you can install [Arcade](https://github.com/pythonarcade/arcade) into the same virtual environment
and run any of it's examples:

```sh
cd <directory of arcade project>
pip install -e .
```

Optimally testing should be done against the `development` branch of Arcade.
In order to enable `arcade-accelerate` add these two lines anytime before importing `arcade`. It is important that
the bootstrap is done prior to importing Arcade, otherwise the monkey-patched functions/classes will not fully apply.

```py
import arcade_accelerate
arcade_accelerate.bootstrap()

import arcade
```

If you would like to run Arcade's test suite with arcade-accelerate enabled, you can do so by setting the `ARCADE_PYTEST_USE_RUST` environment variable before running pytest on Arcade. You just need to ensure that both Arcade and arcade-accelerate are installed in the same environment.
