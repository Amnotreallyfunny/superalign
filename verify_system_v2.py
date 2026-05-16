import superalign
import pyarrow as pa
import os
import json

# Create a sample fasta
fasta_path = "test_sample.fasta"
with open(fasta_path, "w") as f:
    f.write(">Homo sapiens\nACGT\n>mouse\nTGCA\n")

pm = superalign.ProvenanceManager(pipeline_version="0.1.0")

print("--- Testing FASTA Parsing with Provenance ---")
fasta_hash = "fake_fasta_hash_123" # In production, compute from file
parse_id = pm.record_process(
    operation="FASTA_PARSE",
    inputs=[fasta_hash],
    outputs=[] # Will fill after parsing
)

all_entities = []
for entities, metadata in superalign.parse_fasta(fasta_path, batch_size=10):
    all_entities.append(entities)
    
    print("\n--- Testing Reconciliation ---")
    reconciled, provenance = superalign.reconcile(entities, threshold=0.7)
    
    # Record reconciliation in provenance DAG
    reconcile_id = pm.record_process(
        operation="TAXON_RECONCILE",
        inputs=[entities.column("entity_uuid")[0].as_py()], # simplified
        outputs=[reconciled.column("ontology_id")[0].as_py()],
        parent_uuid=parse_id,
        plugin="taxonomy-engine-v1"
    )
    
    print(f"Reconciled {len(entities)} taxa. Reconcile Process ID: {reconcile_id}")

print("\n--- Provenance Report ---")
report_json = pm.to_json_report()
report = json.loads(report_json)
print(json.dumps(report, indent=2))

# Export to Parquet
pm.export_parquet("provenance_manifest.parquet")
print("\nExported provenance to provenance_manifest.parquet")

# Cleanup
os.remove(fasta_path)
