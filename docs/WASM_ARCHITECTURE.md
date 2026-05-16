# SuperAlign: Browser-Native Preprocessing Layer

## Architecture Overview

The browser-native layer of SuperAlign allows researchers to perform data cleaning, taxon reconciliation, and manifest generation locally, ensuring sensitive genomic data never leaves their machine.

### Core Stack
- **Rust WASM**: High-performance streaming parser and metadata extractor.
- **DuckDB-WASM**: Local analytical database for taxonomic ontologies and reconciliation results.
- **Arrow JS**: In-browser memory management for tabular data.
- **IndexedDB**: Persistent storage for session state and large taxonomy caches.

---

## 1. Browser Execution Boundary

| Component | Runtime | Responsibility |
| :--- | :--- | :--- |
| **Streaming Parser** | Rust WASM | Reads FASTA chunks, extracts headers, computes SHA-256 hashes. |
| **Orchestrator** | JavaScript | Manages the File API, coordinates between WASM and DuckDB. |
| **Taxonomy Store** | DuckDB-WASM | Executes fuzzy matching queries against local ontology files. |
| **Manifest Generator** | Rust WASM | Assembles the final deterministic Parquet/JSON manifest. |

---

## 2. Memory Budget & Budgeting Strategy

Genomic files frequently exceed the available RAM in a browser tab (typically ~4GB).

**Strategy: "Windowed Streaming"**
1. **No Buffering**: The JS layer reads the `File` object using `ReadableStream` in 1MB - 10MB chunks.
2. **Stateless WASM Parsing**: The Rust parser processes each chunk independently and returns only the extracted metadata (JSON/Arrow).
3. **Immediate Persistence**: Metadata is immediately inserted into **DuckDB-WASM** (persisted in IndexedDB). This keeps the heap usage stable regardless of input file size.
4. **Target Heap**: < 512MB total RAM usage for the entire preprocessing phase.

---

## 3. Data Flow: Zero-Upload Workflow

1. **User Action**: Drags a 5GB `.fasta` file into the UI.
2. **Parsing**: 
   - JS reads 5MB chunk.
   - Rust WASM parses chunk -> returns `HeaderMetadata[]`.
   - JS inserts metadata into DuckDB.
3. **Reconciliation**:
   - JS triggers SQL queries in DuckDB-WASM to fuzzy-match extracted names against a pre-loaded local taxonomy (NCBI slice).
4. **Export**:
   - User reviews matches.
   - WASM assembles the final `manifest.json` for download.

---

## 4. IndexedDB Integration

IndexedDB is used via the **DuckDB-WASM Filesystem API**:
- **Taxonomy Cache**: The NCBI taxonomy is stored as a persistent `.db` file in IndexedDB.
- **Session State**: Partial parse results are saved to allow resumability if the tab crashes.

---

## 5. Development Guide

To build the WASM core:
```bash
cd crates/wasm-core
wasm-pack build --target web
```
