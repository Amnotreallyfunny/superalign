import superalign
import pyarrow as pa
import json

print("--- Testing Plugin Runtime ---")
runtime = superalign.PluginRuntime()

# Check registered plugins
sigs = runtime.get_signatures()
print(f"Available Plugins: {json.dumps(sigs, indent=2)}")

# Create dummy input data
data = [
    {"raw_label": "Homo sapiens"},
    {"raw_label": "Mus musculus"}
]
table = pa.Table.from_pylist(data)

print("\n--- Executing Plugin: builtin.header_normalizer ---")
result_table = runtime.execute_plugin("builtin.header_normalizer", table)

print("Result Table:")
print(result_table.to_pandas())

# Verify metadata/provenance integration would go here in a full pipeline
