# Development Guide

This document outlines the monorepo architecture, coding standards, and interoperability patterns for SuperAlign.

## 📁 Monorepo Structure

- `/apps`: Final user applications (CLI, UI).
- `/crates`: Rust domain logic crates.
  - `core-parser`: FASTA/FASTQ streaming logic.
  - `taxonomy-engine`: DuckDB-based reconciliation.
  - `matrix-engine`: Zarr out-of-core construction.
  - `provenance-core`: Immutable DAG logging.
  - `schemas`: Canonical Apache Arrow definitions.
- `/python`: Python orchestration layer and bindings.
  - `superalign`: Pure Python API.
  - `superalign-core`: PyO3 bindings crate.

## 🦀 Rust Development

We prioritize **safety**, **determinism**, and **zero-copy**.

### Coding Standards
- **Errors**: Use `anyhow` for applications and `thiserror` for library crates.
- **Async**: Use `tokio` where necessary, but prioritize synchronous code for CPU-bound sequence processing to avoid overhead.
- **Determinism**: Never use non-deterministic sources (e.g., `SystemTime::now()` or random seeds) without logging them to the provenance DAG.

### Schema Contracts
All data movement between crates and languages MUST use the Arrow RecordBatch schemas defined in `crates/schemas`. Direct JSON or raw string exchange is prohibited for sequence data.

## 🐍 Python/Rust Interop (PyO3)

Interoperability is handled via **Arrow IPC Stream**.
1. Rust computes and emiting `RecordBatch`.
2. Rust serializes to in-memory Arrow Stream bytes.
3. Python (PyArrow) consumes the stream with zero-copy.

## 🧪 Testing Strategy

- **Unit Tests**: In every crate/module.
- **Contract Tests**: Verify that Rust and Python see the exact same schema fingerprints.
- **Parity Tests**: Verify that WASM logic matches Native logic bit-for-bit.
- **Golden Tests**: Run full pipelines against curated datasets and compare against hashed snapshots.

## 🚀 Release Process

1. **Versioning**: We follow Semantic Versioning (SemVer).
2. **Changelog**: Automatically generated from Conventional Commits.
3. **Artifacts**:
   - Rust crates published to crates.io.
   - Python wheels (manylinux, macos, win) published to PyPI via GitHub Actions.
   - Docker images published to GHCR.
