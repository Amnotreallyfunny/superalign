# SuperAlign: Scientific Testing & Validation Specification

## 1. Testing Pyramid: Scientific Edition

The SuperAlign testing strategy is built on a hierarchy of determinism. Unlike traditional software, failure in our system is defined by **divergence from expected state**, not just crashes.

| Layer | Responsibility | Runtime/Framework | Failure Type |
| :--- | :--- | :--- | :--- |
| **Scientific CI/CD** | Pipeline gating, reproducibility reports | GitHub Actions | Verification failure |
| **System Parity** | WASM vs Native backend equivalence | Playwright + Pytest | Cross-env divergence |
| **Distributed Stress**| Race conditions, HPC filesystem lockups | Multi-process harness | Nondeterminism |
| **Golden Benchmark** | Curated "known-good" scientific results | Parquet/Zarr snapshots | Accuracy regression |
| **Property/Fuzz** | Noise resilience, label corruption | proptest / Hypothesis | Edge-case OOM |
| **Contract** | Arrow Schema stability, Parquet roundtrip | Rust + PyO3 | Schema drift |
| **Unit** | Atomic logic (parsers, scoring) | cargo-nextest / pytest | Logic error |

---

## 2. Directory Structure for Validation

```text
superalign/
├── tests/
│   ├── golden/              # Versioned curated datasets
│   │   ├── v1_clean/        # 100% matched taxa
│   │   ├── v1_corrupted/    # Simulated biological noise
│   │   └── v1_sparse/       # 90% missing data
│   ├── distributed/         # HPC/Race condition simulations
│   ├── contracts/           # Arrow/Parquet schema validation
│   └── parity/              # WASM vs Backend parity logic
├── benchmarks/              # Throughput and OOM resistance
└── crates/
    └── */src/lib.rs         # Embedded proptests
```

---

## 3. Contract & Schema Validation

To prevent silent corruption at the Polyglot boundary (Rust/Python), we enforce **Strict Schema Fingerprinting**.

**Task:** Every `RecordBatch` generated must pass a `validate_contract()` check that verifies:
1.  **Field Metadata:** Arrow field names, types, and nullability match exactly.
2.  **Timezone Integrity:** All timestamps are `TimeUnit::Microsecond` with `Some("UTC")`.
3.  **UUID Determinism:** UUIDs are generated using a namespaced v5 hash of the (source_file + index) for replayability, or a tracked v4 with provenance logging.

---

## 4. Property-Based Testing: Biological Noise

We use `proptest` (Rust) and `Hypothesis` (Python) to generate adversarial biological labels.

**Property Generator (Example):**
```rust
// Generate corrupted variations of "Homo sapiens"
fn corrupted_label_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        prop::string::string_regex("H(omo)? ?sapiens?").unwrap(),
        prop::string::string_regex("Homo_sapiens_[0-9]{3}").unwrap(),
        prop::string::string_regex("H\\. ?sapiens").unwrap(),
        prop::string::string_regex("[hH]omo [sS]apiens").unwrap(),
    ]
}
```

**Invariant:** `confidence_score` must remain monotonic with respect to edit distance. `Score(Exact) >= Score(Typo) >= Score(Random)`.

---

## 5. Distributed Determinism Framework

In a distributed environment, sequences are processed in an unpredictable order. SuperAlign must guarantee bit-for-bit identity regardless of worker scheduling.

**Determinism Verifier:**
1.  Initialize Zarr matrix $M$.
2.  Run Workers $W_1, W_2, ..., W_n$ with randomized task assignment.
3.  Compute $Hash(M_{final})$.
4.  Re-run with $WorkerCount = 1$.
5.  **Invariant:** $Hash(M_{final}) == Hash(M_{single\_worker})$.

---

## 6. WASM / Native Equivalence (Parity)

We cannot allow the browser preprocessing logic to drift from the HPC backend.

**Parity Harness:**
- **Input:** 1,000 FASTA sequences.
- **Process A:** Run `superalign-core` (Native Rust).
- **Process B:** Run `superalign-wasm` (Node.js/Playwright).
- **Check:** Compare the resulting `RecordBatches`.
- **Allowed Tolerance:** 0.0 (Bitwise identity) for sequence hashes and IDs. $10^{-7}$ for floating-point scores.

---

## 7. Scientific Invariant Engine

These are "sanity checks" to catch non-computational logic failures.

| Invariant | Logic | Failure Action |
| :--- | :--- | :--- |
| **Taxonomic Boundary** | Species $X$ cannot have a lineage including Kingdom $Y$ if $X$ is known as $Z$. | CRITICAL - Hard stop |
| **Matrix Density** | Row $R$ cannot be 100% gaps if Taxon $T$ was marked as present. | ERROR - Log & Stop |
| **Provenance Link** | Every UUID in the matrix must exist in the Provenance DAG. | CRITICAL - Data Poisoning |
| **Temporal Flow** | $T_{reconcile} > T_{parse}$. | WARNING - Clock drift |

---

## 8. Failure & Corruption Simulation (Chaos Engineering)

We proactively simulate failures to test the **Resumability Engine** and **Deterministic Policy Engine**.

1.  **Truncated Ingestion:** Terminate FASTA parsing at 50.5%.
2.  **Duplicate UUID Injection:** Inject a duplicate ID into a batch to test collision handling.
3.  **Conflict Injection:** Ingest a "Poisoned FASTA" containing intentional naming collisions and synonym loops to verify that the `Ambiguity Management` layer correctly flags them for review.
4.  **Ontology Mismatch:** Attempt to reconcile against a v2 manifest using a v1 taxonomy database.
5.  **Zarr Chunk Corruption:** Manually flip bits in a `.zarr` chunk file. SuperAlign must detect the checksum mismatch and refuse to export the supermatrix.

---

## 9. Scientific CI/CD Pipeline Design

```yaml
stages:
  - lint: [ruff, cargo-fmt, clippy]
  - unit-tests: [cargo-nextest, pytest]
  - contract-validation: [check-arrow-schemas, check-parquet-roundtrip]
  - property-validation: [reconciliation-fuzzing, noise-resilience]
  - golden-regression: [v1-benchmark-comparison]
  - distributed-stress: [random-worker-interleaving, thread-count-sweep]
  - wasm-parity: [browser-vs-native-snapshot]
  - performance-audit: [check-throughput-thresholds]
```

---

## 10. Gating & Release Strategy

1.  **Strict Gating:** No PR is merged if `golden-regression` or `wasm-parity` fails.
2.  **Reproducibility Report:** Every release build generates a `ValidationReport.json` containing the hashes of all golden datasets and their pass status.
3.  **Telemetry:** None. We rely on community-submitted failure reports containing the `superalign_run.json` manifest.
