# SuperAlign Monorepo Architecture

## Overview
SuperAlign is a high-performance computational biology platform built with a "Rust-core, Python-orchestration" philosophy. This monorepo manages multiple Rust crates, a Python package, and cross-language bindings.

## Directory Structure
- `/apps`: End-user applications.
  - `/cli`: Rust-based native CLI.
  - `/wasm-ui`: Future browser-based interface.
- `/crates`: Core domain logic (Rust).
  - `core-parser`: Streaming FASTA/NEXUS parsing.
  - `taxonomy-engine`: DuckDB-backed reconciliation.
  - `provenance-core`: Immutable event logging.
  - `matrix-engine`: Zarr-based out-of-core matrix construction.
- `/python`: Python orchestration layer.
  - `superalign`: Pure Python logic and CLI.
  - `superalign-core`: PyO3 bindings to Rust crates.
- `/schemas`: Shared data contracts (Arrow/Parquet).
- `/tests`: Cross-language integration tests.

## Development Workflow

### Rust
Use `cargo` for crate-level development:
```bash
cargo build
cargo test
```

### Python & Bindings
We use `maturin` to build Rust bindings into the Python package:
```bash
# Develop mode
maturin develop
```

### Linting & Formatting
- **Rust:** `cargo fmt` and `cargo clippy`.
- **Python:** `ruff check` and `ruff format`.

## Technical Standards
1. **Zero-Copy Data:** Use Apache Arrow for high-throughput data exchange between Rust and Python.
2. **Deterministic Provenance:** All transformations must be logged via `provenance-core`.
3. **Out-of-Core by Default:** Use Zarr for matrices and DuckDB for metadata to support datasets exceeding available RAM.
