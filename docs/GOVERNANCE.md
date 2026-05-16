# SuperAlign Repository Governance

## 1. Branch Protection Rules (Main Branch)

The `main` branch is protected with the following requirements:
- **No Direct Pushing**: All changes must be via Pull Request.
- **Required Approvals**: Minimum of 1 approval from a member of the `@superalign-core` team.
- **Required Status Checks**:
  - `Scientific Validation CI` (Rust/Python tests)
  - `Documentation Validation` (README examples)
  - `Contract Validation` (Arrow schemas)
- **Linear History**: Only squash or rebase merges allowed.
- **Signed Commits**: Mandatory GPG signing for all contributors.

## 2. CODEOWNERS Enforcement
The `.github/CODEOWNERS` file is used to automatically assign reviewers. Schema changes (`/crates/schemas`) require explicit approval from the Platform Architect.

## 3. Release Strategy
SuperAlign follows **Semantic Versioning (SemVer)**.

### Release Branching Model
- `main`: Current development state.
- `release/vX.Y.Z`: Dedicated branch for release stabilization.
- `tag vX.Y.Z`: Immutable point-in-time snapshot.

### Release Workflow
1. Create a release PR: `chore: prepare release vX.Y.Z`.
2. Merge to `main`.
3. CI automatically triggers a GitHub Release and publishes artifacts (PyPI, crates.io, Docker).

## 4. Reproducibility Policy
- Any change affecting the `matrix-engine` or `taxonomy-engine` must be accompanied by a reproducibility report.
- The `ValidationReport.json` from the Golden Dataset suite must be attached to the PR.
