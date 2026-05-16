import superalign
import os
import shutil

store_path = "test_matrix.zarr"
if os.path.exists(store_path):
    shutil.rmtree(store_path)

matrix = superalign.MatrixEngine(store_path)

print("--- Testing Matrix Creation ---")
# 3 taxa, 20 length
matrix.create_matrix(num_taxa=3, total_length=20)
print(f"Matrix created at {store_path}")

print("\n--- Writing Sequence Data ---")
# Write data for Taxon 0
matrix.write_chunk(taxon_index=0, start_pos=0, data=b"ATGCATGC")
# Write data for Taxon 1 at an offset
matrix.write_chunk(taxon_index=1, start_pos=5, data=b"GGGGGG")

print("Data written to Zarr.")

# In production, we'd use zarr-python to read it back
import zarr
import numpy as np

z = zarr.open(store_path, mode='r')
# np.array(z) should work for Zarr arrays
arr = z[:]
print(f"\nMatrix Shape: {arr.shape}")
print("Matrix Content (as ASCII codes):")
print(arr)

# Cleanup
# shutil.rmtree(store_path)
