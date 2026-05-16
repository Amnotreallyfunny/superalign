# Contributing to SuperAlign

Thank you for your interest in SuperAlign! This project aims for scientific rigor and computational excellence.

## 📜 Contribution Standards

### 1. Developer Certificate of Origin (DCO)
We do not use a CLA. Instead, we use the DCO. By signing off your commits, you certify that you have the right to submit the code under the Apache 2.0 license.

**How to sign off:**
Add `-s` to your commit command:
```bash
git commit -s -m "feat: added new reconciliation algorithm"
```

### 2. Commit Conventions
We use **Conventional Commits**:
- `feat:`: A new feature.
- `fix:`: A bug fix.
- `chore:`: Maintenance or configuration.
- `docs:`: Documentation changes.
- `perf:`: Performance improvements.

### 3. Workflow
1. Fork the repository.
2. Create a branch: `git checkout -b feat/my-cool-feature`.
3. Make your changes and add tests.
4. Run `make lint` and `make test`.
5. Push and open a Pull Request.

---

## 🔬 Pull Request Requirements

Every PR must meet the following criteria to be merged:
- **Approval**: Minimum of 1 approval from a core maintainer.
- **CI**: All status checks must pass (Lint, Unit Tests, Contract Validation).
- **Reproducibility**: If you modify the matrix or reconciliation engines, you must run the Golden Dataset suite to ensure no regression in output determinism.
- **Provenance**: New components must correctly emit provenance records matching the canonical schemas.

## 🛡 Security
Please report security vulnerabilities to `security@superalign.io` rather than opening a public issue. See [SECURITY.md](SECURITY.md) for more details.
