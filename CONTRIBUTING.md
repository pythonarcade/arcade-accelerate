# Dev Environment Setup

These instructions are one *possible* way to get everything you need: python, rust, venv, etc.
There are many other ways to setup a python dev environment. You are free to deviate from this guide.
These preliminary steps should be completed before the steps in README.md.

Steps prefixed with `[NameOfTool]` only apply if you use that tool, skip them otherwise.

### [WSL2] install GPU drivers

If you want to run arcade from within WSL2, you may need to install the necessary GPU support drivers.

https://learn.microsoft.com/en-us/windows/wsl/tutorials/gui-apps

### Install rust via rustup

There is a one-liner command you can copy-paste on their website:
https://www.rust-lang.org/tools/install

This gives you rust and cargo.

On Windows, use the exe.  On Linux and WSL2, use the shell one-liner.

### Install rye

This tool can download python versions and create venvs.
Many other tools exist to install python and venvs.  This one can download a
binary distribution of python 3.11 with very little effort.

https://github.com/mitsuhiko/rye

```
cargo install --git https://github.com/mitsuhiko/rye rye
```

### Use rye to download the latest python and create a venv

```
rye pin cpython@3.11
rye add --dev pip
rye sync
```

Now you have python 3.11 and pip in a venv at `.venv`

### [VSCode] Restart VSCode

The integrated terminal should automatically notice that you have a venv.  Open the
terminal.  Each line should start with `(.venv)` prefix to show that it's
loaded. Type `which python` and `python --version` to verify that it's using the
correct python.

### Clone arcade, install it into venv

This step can be skipped or modified if you have already cloned `arcade` elsewhere.

*We intentionally avoid cloning into a subdirectory named `arcade` so that `import arcade`
does not get confused.*

```
git clone https://github.com/pythonarcade/arcade arcade-git
pip install -e './arcade-git/[dev]'
# Test that it worked
python -m arcade
```

### Build and run

Follow the instructions in `README.md`.