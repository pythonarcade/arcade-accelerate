.PHONY: lint-python
lint-python:
	ruff python
	black --check --diff python

.PHONY: lint-rust
lint-rust:
	cargo fmt --version
	cargo fmt --all -- --check
	cargo clippy --version
	cargo clippy -- -D warnings -W clippy::dbg_macro -W clippy::print_stdout

.PHONY: lint
lint: lint-python lint-rust

.PHONY: test
test:
	cargo test

.PHONY: fmt
fmt: fmt-python fmt-rust

.PHONY: fmt-python
fmt-python:
	black python

.PHONY: fmt-rust
fmt-rust:
	cargo fmt