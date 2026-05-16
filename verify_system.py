import os

import superalign

# Create a sample fasta
fasta_path = "test_sample.fasta"
with open(fasta_path, "w") as f:
    f.write(">Homo sapiens\nACGT\n>mouse\nTGCA\n>unknown_species\nGGGG\n")

print("--- Testing FASTA Parsing ---")
for entities, metadata in superalign.parse_fasta(fasta_path, batch_size=10):
    print(f"Entities:\n{entities.to_pandas()[['entity_uuid', 'raw_label']]}")
    print(f"Metadata:\n{metadata.to_pandas()}")

    print("\n--- Testing Reconciliation ---")
    reconciled, provenance = superalign.reconcile(entities, threshold=0.7)

    # Join reconciled info back to entities for display
    # (In production we'd do a proper join)
    print(f"Reconciled Table:\n{reconciled.to_pandas()}")
    cols = ["matching_algorithm", "computed_distance", "selected_candidate"]
    print(f"Provenance Table:\n{provenance.to_pandas()[cols]}")

# Cleanup
os.remove(fasta_path)
