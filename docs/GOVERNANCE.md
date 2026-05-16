# SuperAlign: Open Source Governance & Licensing Specification

## 1. License Strategy Analysis

| License | Patent Grant | Viral Risk | Commercial Adoption | Academic/Sci Preference | Rationale for SuperAlign |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Apache 2.0** | **Yes** | **None** | **High** | **High** | **Recommended.** Explicit patent grants are mandatory for Pharma/Biotech adoption. |
| **MIT / BSD-3**| No | None | Very High | Very High | Lacks patent protection; risky for a platform handling genomic IP. |
| **MPL 2.0** | Yes | Weak | High | Medium | Good for libraries, but less standard for infrastructure orchestration. |
| **GPLv3** | Yes | Strong | Low | Medium | Viral nature prevents proprietary plugin ecosystems (e.g., private Pharma ML models). |
| **AGPL** | Yes | Critical | Very Low | Low | Cloud/SaaS "loophole" closure is too restrictive for distributed HPC nodes. |

### Final Recommendation: Apache License 2.0
**Rationale:** SuperAlign is a "Foundational Infrastructure" project. Much like Kubernetes, Spark, and Arrow, we require a license that provides "Patent Peace" for large pharmaceutical contributors while remaining permissive enough to allow a diverse ecosystem of both open-source and proprietary plugins.

---

## 2. Core Repository Licensing Structure

SuperAlign will follow the **SPDX (Software Package Data Exchange)** standard for all source files.

**Header Template:**
```rust
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 SuperAlign Contributors
```

**Repository Legal Files:**
1. `LICENSE`: Full text of Apache 2.0.
2. `NOTICE`: Required by Apache 2.0; contains attribution for the core team and heavy dependencies (Arrow, DuckDB, Zarr).
3. `CITATION.cff`: Standard format for academic citation (mapped to Zenodo DOI).

---

## 3. Contributor Governance Model

We recommend an **Open Steering Committee** model, transitioning to a foundation (e.g., **NumFOCUS** or **Linux Foundation**) upon reaching Phase 3.

### Contribution Workflow: The DCO Approach
To maximize contributor velocity while ensuring legal provenance, SuperAlign will use the **Developer Certificate of Origin (DCO)** rather than a heavy CLA.
*   **Enforcement:** Every commit must be signed off (`git commit -s`).
*   **Rationale:** Standard practice for Linux, Git, and Docker. It provides a lightweight audit trail of authorship without the friction of a corporate-signed legal document.

---

## 4. Third-Party Dependency Policy

To prevent "License Poisoning," SuperAlign implements an automated **CI Compliance Gate**.

**Allow-List:**
- Permissive: Apache 2.0, MIT, BSD, ISC, Unlicense.
- Weak Copyleft: MPL 2.0 (provided they remain isolated crates/packages).

**Prohibited (without explicit waiver):**
- Strong Copyleft: GPL, AGPL, LGPL (risk of viral leakage into the orchestration layer).

**CI Automation:**
- **Rust:** `cargo-deny` to audit crate licenses and security vulnerabilities.
- **Python:** `pip-licenses` to verify the environment.

---

## 5. Dataset & Ontology Licensing

Scientific metadata requires a distinct legal strategy from code.

| Data Type | Example | Recommended License | Policy |
| :--- | :--- | :--- | :--- |
| **Ontologies** | NCBI Taxonomy | Public Domain | SuperAlign provides "Attribution-only" scripts; no re-licensing. |
| **Benchmark Corpuses** | Golden Datasets | **CC-BY-4.0** | Encourages sharing while mandating academic citation. |
| **Manifests** | Parquet Outputs | **CC0 (No Rights Reserved)**| Provenance trails should be unencumbered to ensure data flow. |

---

## 6. Plugin Ecosystem Boundaries

To allow the growth of a commercial ecosystem around SuperAlign:
1. **The SDK:** The `plugin-runtime` crate and Python SDK are Apache 2.0.
2. **The Boundary:** Interactions occur via **Arrow IPC** (in-memory) or **Parquet** (disk).
3. **Licensing:** Plugins are considered "Derivative Works" only if they link statically to the core. Since SuperAlign uses IPC boundaries, **proprietary and GPL plugins can coexist** without contaminating the core platform.

---

## 7. Scientific Reproducibility Governance

Reproducibility is a legal requirement for regulatory-grade science (FDA/EMA).
- **Deterministic Releases:** Every release is cross-checked against the "Golden Dataset" suite.
- **Schema Stability:** The `superalign-schemas` crate is versioned via **SemVer**. Breaking changes to schemas require a minimum 6-month deprecation period.
- **Provenance Requirement:** All "Official" SuperAlign plugins must output the `MatchProvenance` Arrow batch to be certified.

---

## 8. Security & Disclosure Policy

Located in `SECURITY.md`:
- **Vulnerability Handling:** Private disclosure via `security@superalign.io` or GitHub Private Vulnerability Reporting.
- **Disclosure Timeline:** 90-day coordinated disclosure policy.
- **WASM Security:** Explicit notice that WASM preprocessing is "Local-First" and sandboxed by the browser's security model.

---

## 9. Future Commercialization Roadmap

SuperAlign preserves future optionality for a **"SaaS Orchestration"** or **"Enterprise Support"** model:
- **Core Platform:** Remains Apache 2.0 (OSS).
- **Commercial Extensions:** Cloud-native UI, SSO integrations, or high-density hosting services can be offered as proprietary additions without modifying the core license.
- **Risk Mitigation:** By owning the trademark "SuperAlign" through a non-profit foundation early, we prevent single-vendor capture.

---

## 10. Long-Term Technical Debt: Licensing Traps

- **Dependency Creep:** We must avoid dependencies that switch to "Business Source Licenses" (BSL). **Action:** Quarterly audit of `Cargo.lock` and `requirements.txt`.
- **Contribution Provenance:** Failure to enforce DCO early will make it impossible to move to a foundation later. **Action:** Enable "DCO Check" GitHub App immediately.
