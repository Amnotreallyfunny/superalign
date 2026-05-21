import hashlib
import os

import numpy as np
import zarr


def compute_zarr_checksum(store_path: str) -> str:
    """Computes a recursive SHA-256 hash of all data chunks in a Zarr store."""
    hasher = hashlib.sha256()
    for root, _dirs, files in os.walk(store_path):
        for names in sorted(files):
            filepath = os.path.join(root, names)
            with open(filepath, "rb") as f:
                hasher.update(f.read())
    return hasher.hexdigest()


def validate_matrix_integrity(store_path: str) -> str:
    print(f"--- SUPERALIGN INTEGRITY AUDIT: {store_path} ---")

    # 1. Zarr Open
    z = zarr.open(store_path, mode="r")
    print(f"Matrix Shape: {z.shape}")

    # 2. Scientific Invariants
    # Check for excessive gap density (poisoning)
    for i in range(z.shape[0]):
        row = z[i, :]
        gap_ratio = np.mean(row == ord("-"))
        if gap_ratio > 0.99:
            print(
                f"WARNING: Taxon at index {i} has extreme gap density "
                f"({gap_ratio*100:.2f}%)"
            )

    # 3. Determinism
    checksum = compute_zarr_checksum(store_path)
    print(f"Scientific Checksum: {checksum}")
    return checksum
