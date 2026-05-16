# Installation Guide

SuperAlign is a polyglot platform. Installation requirements vary depending on whether you are an end-user, a developer, or an HPC administrator.

## 1. End-User Installation (Python)

Recommended for most researchers.

```bash
# We recommend using a virtual environment
python -m venv .venv
source .venv/bin/activate

pip install superalign
```

## 2. Developer Installation (Monorepo)

Required if you are contributing to the Rust core or Python bindings.

### Prerequisites
- **Rust Toolchain**: [Install via rustup](https://rustup.rs/) (Stable 1.75+)
- **Python**: 3.9+
- **Maturin**: `pip install maturin`

### Local Setup
```bash
git clone https://github.com/Amnotreallyfunny/superalign.git
cd superalign

# 1. Install Python dependencies
pip install -e ".[dev]"

# 2. Build and link Rust bindings
maturin develop
```

## 3. HPC Cluster Setup

SuperAlign is designed to run in distributed environments via Nextflow or Snakemake.

### Containerized Execution (Recommended)
We provide Docker and Singularity images for stable execution.

```bash
# Docker
docker pull ghcr.io/superalign/superalign:latest

# Singularity
singularity pull docker://ghcr.io/superalign/superalign:latest
```

### Nextflow Integration
Add the following to your `nextflow.config`:

```nextflow
process {
    withName: 'SUPERALIGN_.*' {
        container = 'ghcr.io/superalign/superalign:latest'
    }
}
```

---

## Dependency Verification
To verify your installation is correct:
```bash
superalign --version
```
Or run the internal diagnostic:
```python
import superalign
print(superalign.schema_version())
```
