# SuperAlign

[![CI](https://github.com/Amnotreallyfunny/superalign/actions/workflows/ci.yml/badge.svg)](https://github.com/Amnotreallyfunny/superalign/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

**SuperAlign** is a production-grade **Deterministic Phylogenomics Infrastructure** platform. It provides a high-performance orchestration layer for sequence preprocessing, hierarchical taxon reconciliation, and bounded-memory sparse matrix construction.

## 🔬 Core Vision
SuperAlign bridges the gap between raw genomic data and tree-building engines. It moves beyond ad-hoc scripts by enforcing:
- **Biological Identity**: Prioritizing TaxIDs and Accession grounding over fragile string similarity.
- **Bit-for-bit Reproducibility**: Identical outputs for identical inputs across environments.
- **Immutable Provenance**: Cryptographic DAG-based event logging of every transformation rationale.
- **Bounded-Memory Processing**: Indexing 10M+ taxa on hardware with minimal RAM using a tiered persistent index strategy.

---

## 🏗 Architecture
SuperAlign uses a **Polyglot Core** strategy with a tiered memory model:
- **Rust (Backend)**: Hierarchical reconciliation, persistent indexing (DuckDB), and Zarr I/O.
- **Python (Orchestration)**: API, workflow logic, and data science integration.
- **Hot Cache (LRU)**: Bounded-memory lookups for high-throughput stream processing.

```text
[FASTA] -> [Metadata Extractor] -> [TaxID/Accession Grounding] -> [Persistent Index (DuckDB)]
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
    # 2. Hierarchical Reconcile
    reconciled, provenance = superalign.reconcile(entities)
    
    # 3. Assemble SuperMatrix (Bounded-memory)
    matrix = superalign.MatrixEngine("output/supermatrix.zarr")
    # ... assembly logic ...
```

---

## 🛠 Feature Matrix

| Feature | Status | Description |
| :--- | :--- | :--- |
| **Hierarchical Reconciler** | ✅ Production | TaxID-first resolution with deterministic tie-breaking. |
| **Persistent Indexer** | ✅ Production | DuckDB-backed O(log N) lookups for 10M+ taxa. |
| **Matrix Engine** | ✅ Production | Zarr-backed out-of-core sparse matrix builder. |
| **Provenance Core** | ✅ Production | Immutable event logging with explanation tracking. |
| **Ambiguity Queue** | 🚧 Beta | Scientific isolation of conflicting records for review. |
| **WASM Core** | 🚧 Beta | Browser-native parsing and reconciliation. |

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
