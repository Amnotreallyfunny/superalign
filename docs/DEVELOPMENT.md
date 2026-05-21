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

We prioritize **safety**, **determinism**, and **bounded-memory scale**.

### Tiered Memory Model
SuperAlign is designed for RefSeq-scale runs (10M+ taxa). To maintain a bounded memory footprint:
1. **Persistent Index (DuckDB)**: Global taxon and locus mappings are stored on-disk.
2. **Hot Cache (LRU)**: Frequently accessed coordinate mappings are cached in memory (default 10k entries).
3. **Zarr Out-of-Core**: Sequence data is written directly to chunked Zarr storage without loading the entire matrix into RAM.

### Hierarchical Reconciliation
The `taxonomy-engine` follows a strict deterministic priority hierarchy:
1. **Biological IDs**: Recursive Regex extraction for NCBI TaxIDs (`taxid:9606`).
2. **Accession Grounding**: Mapping GenBank/RefSeq accessions to canonical TaxIDs.
3. **Exact Synonym**: Direct matching against the DuckDB ontology table.
4. **Fuzzy Fallback**: Jaro-Winkler similarity used ONLY as assistive fallback.

### Ambiguity Management
Conflicting matches (equal scores/ranks) MUST NOT be automatically resolved. They are assigned the `ISOLATED_PENDING_REVIEW` state and isolated from downstream matrix assembly.

### Coding Standards
- **Errors**: Use `anyhow` for applications and `thiserror` for library crates.
- **Async**: Use `tokio` for I/O and orchestration, but prioritize synchronous code for CPU-bound sequence processing.
- **Determinism**: Every transformation rationale is logged. Tie-breaking rules are policy-based and immutable.

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
