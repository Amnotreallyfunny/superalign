import superalign
import os
import shutil
import json

store_path = "dist_matrix.zarr"
if os.path.exists(store_path):
    shutil.rmtree(store_path)

matrix = superalign.MatrixEngine(store_path)

print("--- Step 1: Dry-run Planning ---")
taxa = ["Human", "Mouse", "FruitFly"]
# Mapping loci to their lengths
loci = [("BRCA1", 15), ("GAPDH", 10), ("COX1", 20)]

plan = matrix.plan_matrix(taxa, loci)
print(f"Plan generated. Total Length: {json.loads(plan.to_json())['total_length']}")

print("\n--- Step 2: Pre-allocation (HPC safe) ---")
matrix.initialize_from_plan(plan)
print(f"Matrix initialized at {store_path}")

print("\n--- Step 3: Simulating Distributed Writes (Lock-free) ---")
# Worker 1: Human sequences
matrix.write_taxon_locus(plan, "Human", "BRCA1", b"ATGCATGCATGCATG")
matrix.write_taxon_locus(plan, "Human", "GAPDH", b"GGGGGGGGGG")

# Worker 2: Mouse sequences
matrix.write_taxon_locus(plan, "Mouse", "BRCA1", b"TTTTTTTTTTTTTTT")
matrix.write_taxon_locus(plan, "Mouse", "COX1",  b"CCCCCCCCCCCCCCCCCCCC")

print("Distributed data written to Zarr.")

# Verification read
import zarr
z = zarr.open(store_path, mode='r')
arr = z[:]
print("\nFinal Matrix Content:")
print(arr)

# Metadata verification
print(f"\nMatrix Shape (Taxa, Loci): {arr.shape}")
