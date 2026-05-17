# SuperAlign: Bioinformatics CLI Guide

This guide provides real-world scenarios and CLI workflows for bio-engineers and phylogenetics researchers. SuperAlign is designed for **Scalability**, **Reproducibility**, and **Local-First** data privacy.

---

## 🧬 Scenario 1: Pathogen Surveillance (Viral/Fungal)
**Objective**: Ingest a "messy" FASTA from a clinical environment and map it to standardized taxonomic IDs.

### 1. Ingest clinical sequences
Extract internal UUIDs and compute SHA-256 identities without uploading data to the cloud.
```bash
superalign ingest clinical_isolates.fasta --batch-size 2048
```

### 2. Reconcile with Fuzzy-Matching
Automatically map clinical headers like `hCoV-19/USA/CA-1234/2021` to **SARS-CoV-2** using the explainable AI engine.
```bash
superalign reconcile clinical_isolates.fasta --threshold 0.85
```
*   **Bio-Tip**: If your scores are too low (< 0.8), check for sample contamination or mismatched delimiters in your FASTA headers.

---

## 🏗 Scenario 2: Large-Scale Phylogenomic Assembly
**Objective**: Construct a sparse supermatrix from 10,000+ loci for a species tree.

### 1. Pre-allocate the Zarr Store
Instead of giant in-memory strings, SuperAlign uses **Zarr** for out-of-core storage.
```bash
superalign build-matrix --output hominidae_matrix.zarr
```

### 2. Export for Legacy Tree-Builders
Flatten the modern Zarr store into a legacy format compatible with **RAxML**, **IQ-TREE**, or **MrBayes**.
```bash
superalign export hominidae_matrix.zarr --output final_alignment.fasta
```

---

## 🛡 Scenario 3: Peer Review & Audit
**Objective**: Verify that a published dataset hasn't been tampered with and is 100% reproducible.

### 1. Verify a Provenance Manifest
Run the verification engine against a collaborator's `.parquet` manifest to prove bit-identity of the pipeline run.
```bash
superalign verify results_provenance.parquet
```

### 2. Audit Matrix Density
Check if your supermatrix contains "poison" sequences (all gaps or random noise).
```python
# Via the internal scientific validator
from tests.validate_scientific_integrity import validate_matrix_integrity
validate_matrix_integrity("my_research.zarr")
```

---

## 🛠 Command Reference Table

| Command | Capability | Biological Context |
| :--- | :--- | :--- |
| `ingest` | Streaming Parser | Hashing raw FASTA/FASTQ sequence data. |
| `reconcile`| Ontology Mapping | Fuzzy-matching to NCBI/Pango taxonomies. |
| `build-matrix`| Constructor | Assembling loci into a Zarr supermatrix. |
| `export` | Ecosystem Bridge | Generating FASTA/PHYLIP for tree building. |
| `verify` | Reproducibility | Verifying bit-identity of a pipeline run. |

---

## 🧪 Technical Invariants for Bio-Engineers
- **Zero-Upload**: All sequence hashing and taxonomic matching occurs in the **Rust/WASM sandbox**.
- **Deterministic**: Repeated runs on the same input FASTA will generate identical matrix hashes.
- **Out-of-Core**: Memory usage is constant ($O(1)$) regardless of whether you process 10 sequences or 100,000.
