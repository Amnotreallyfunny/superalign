import io
import pyarrow as pa
from . import core

def parse_fasta(path: str, batch_size: int = 1024):
    """
    Parses a FASTA file using the high-performance Rust engine.
    Yields pairs of (taxon_entities, sequence_metadata) as PyArrow Tables.
    """
    raw_results = core.parse_fasta(path, batch_size)
    for res in raw_results:
        entities_buf = io.BytesIO(res["taxon_entities"])
        metadata_buf = io.BytesIO(res["sequence_metadata"])
        
        entities_table = pa.ipc.open_stream(entities_buf).read_all()
        metadata_table = pa.ipc.open_stream(metadata_buf).read_all()
        
        yield entities_table, metadata_table

def reconcile(table: pa.Table, threshold: f64 = 0.8):
    """
    Reconciles a table of TaxonEntity records against the ontology.
    Returns (reconciled_table, provenance_table).
    """
    sink = io.BytesIO()
    with pa.ipc.new_stream(sink, table.schema) as writer:
        writer.write_table(table)
    
    batch_bytes = sink.getvalue()
    res = core.reconcile_batch(batch_bytes, threshold)
    
    reconciled_buf = io.BytesIO(res["reconciled"])
    provenance_buf = io.BytesIO(res["provenance"])
    
    reconciled_table = pa.ipc.open_stream(reconciled_buf).read_all()
    provenance_table = pa.ipc.open_stream(provenance_buf).read_all()
    
    return reconciled_table, provenance_table

class ProvenanceManager:
    def __init__(self, pipeline_version: str):
        self._inner = core.PyProvenanceManager(pipeline_version)
    
    def record_process(self, operation: str, inputs: list[str], outputs: list[str], parent_uuid: str = None, plugin: str = None):
        return self._inner.record_process(operation, inputs, outputs, parent_uuid, plugin)
    
    def to_json_report(self):
        return self._inner.to_json_report()
    
    def export_parquet(self, path: str):
        return self._inner.export_parquet(path)

class MatrixEngine:
    def __init__(self, store_path: str):
        self._inner = core.PyMatrixEngine(store_path)
    
    def create_matrix(self, num_taxa: int, total_length: int):
        return self._inner.create_matrix(num_taxa, total_length)
    
    def write_chunk(self, taxon_index: int, start_pos: int, data: bytes):
        return self._inner.write_chunk(taxon_index, start_pos, data)
