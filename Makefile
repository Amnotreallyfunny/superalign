.PHONY: help install build-rust develop test lint clean

help:
	@echo "SuperAlign Monorepo Commands:"
	@echo "  install      Install python dev dependencies"
	@echo "  build-rust   Build all rust crates"
	@echo "  develop      Build rust bindings and install in editable mode"
	@echo "  test         Run all tests (rust + python)"
	@echo "  lint         Run all linters"

install:
	pip install -e ".[dev]"

build-rust:
	cargo build --workspace

develop:
	maturin develop

test:
	cargo test --workspace
	pytest

lint:
	cargo fmt --all -- --check
	cargo clippy --workspace -- -D warnings
	ruff check .
	mypy python/superalign

clean:
	cargo clean
	find . -type d -name "__pycache__" -exec rm -rf {} +
