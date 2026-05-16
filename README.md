# SuperAlign

[![CI](https://github.com/Amnotreallyfunny/superalign/actions/workflows/ci.yml/badge.svg)](https://github.com/Amnotreallyfunny/superalign/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

**SuperAlign** is a production-grade computational biology infrastructure platform for scalable phylogenomics. It provides a high-performance orchestration layer for sequence preprocessing, taxon reconciliation, and out-of-core sparse matrix construction.

## 🔬 Core Vision
SuperAlign bridges the gap between raw genomic data and tree-building engines. It moves beyond ad-hoc scripts by enforcing:
- **Bit-for-bit Reproducibility**: Identical outputs for identical inputs across environments.
- **Immutable Provenance**: Automated DAG-based event logging of every transformation.
- **Scalable Out-of-Core Processing**: Handle 100,000+ genomes on hardware with minimal RAM using Zarr and DuckDB.
- **WASM Preprocessing**: Local-first data cleaning entirely in the browser.

---

## 🏗 Architecture
SuperAlign uses a **Polyglot Core** strategy:
- **Rust (Backend)**: Parsing, string-distance matching, and Zarr I/O.
- **Python (Orchestration)**: API, workflow logic, and data science integration.
- **Apache Arrow**: The strict, zero-copy data contract between all layers.

```text
[FASTA] -> [Rust Parser] -> [Arrow RecordBatch] -> [Taxonomy Engine (DuckDB)]
                                    |
                                    v
[Zarr Matrix] <- [Matrix Engine] <- [Provenance Core (DAG Tracing)]
```

---

## 🚀 Quickstart

### 1. Installation
```bash
pip install superalign
```

### 2. High-Level Usage (Python)
```python
import superalign
import pyarrow as pa

# 1. Streaming Parse
for entities, metadata in superalign.parse_fasta("data/samples.fasta"):
    # 2. Reconcile against Taxonomy
    reconciled, provenance = superalign.reconcile(entities)
    
    # 3. Assemble SuperMatrix (Out-of-core)
    matrix = superalign.MatrixEngine("output/supermatrix.zarr")
    # ... assembly logic ...
```

---

## 🛠 Feature Matrix

| Feature | Status | Description |
| :--- | :--- | :--- |
| **Streaming Parser** | ✅ Production | Multi-GB FASTA support with SHA-256 hashing. |
| **Taxon Reconciler** | ✅ Production | DuckDB-backed fuzzy matching with Jaro-Winkler. |
| **Matrix Engine** | ✅ Production | Zarr-backed out-of-core sparse matrix builder. |
| **Provenance Core** | ✅ Production | Parquet-based immutable event logging. |
| **WASM Core** | 🚧 Beta | Browser-native parsing and reconciliation. |
| **Plugin Runtime** | 🚧 Alpha | Sandboxed pure-function extension system. |

---

## 📖 Documentation
- [**Installation Guide**](docs/INSTALL.md) - Cluster, Developer, and User setup.
- [**Development Guide**](docs/DEVELOPMENT.md) - Monorepo architecture and Rust/Python internals.
- [**Governance**](docs/GOVERNANCE.md) - Licensing, Steering Committee, and DCO.
- [**Testing Strategy**](docs/TESTING_STRATEGY.md) - Scientific validation and determinism checks.

---

## 🤝 Contributing
We welcome contributions! Please see our [**Contributing Guidelines**](CONTRIBUTING.md).
Note: We enforce a **Developer Certificate of Origin (DCO)**. All commits must be signed off (`git commit -s`).

---

## ⚖️ License
SuperAlign is licensed under the **Apache License, Version 2.0**. See [LICENSE](LICENSE) for details.
Copyright (c) 2026 SuperAlign Contributors.
