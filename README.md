
# arcade-accelerate

An experimental library for accelerating arcade using rust. The module can
be imported and monkey patch arcade replacing functions and types with rust
versions.

```py
import arcade_accelerate
arcade_accelerate.bootstrap()
```

## Build / Setup

* Install maturin
  * `pip install maturin`
  * (Package crated with 0.14.15)
* Install the arcade version you are comparing with
  * Preferably install from source in editable mode

Install the create as module in the current virtualenv

```sh
# debug
maturin develop
# release
maturin develop --release
```

Build python package for release

```bash
# wheel
maturin build -i python --release
# sdist
maturin build -i python --release --sdist
```

When performance testing always use the release build.

## Info

This project has a python module and a rust module.

* `arcade_accelerate` - python module
* `arcade_accelerate_rust` - rust module

The python module just contains some helper functions to bootstrap the
acceleration.

The `tests` directory contains some performance tests.

# Resources

* [pyo3 user guide](https://pyo3.rs)
* [maturin user guide](https://www.maturin.rs/)
