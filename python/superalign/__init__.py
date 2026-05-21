import io
import json
from typing import Any, Iterator, Optional

import pyarrow as pa

from . import core  # type: ignore

__all__ = [
    "core",
    "parse_fasta",
    "reconcile",
    "ProvenanceManager",
    "WritePlan",
    "MatrixEngine",
    "PluginRuntime",
]


def parse_fasta(
    path: str, batch_size: int = 1024
) -> Iterator[tuple[pa.Table, pa.Table]]:
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


def reconcile(table: pa.Table, threshold: float = 0.8) -> tuple[pa.Table, pa.Table]:
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
    def __init__(self, pipeline_version: str) -> None:
        self._inner = core.PyProvenanceManager(pipeline_version)

    def record_process(
        self,
        operation: str,
        inputs: list[str],
        outputs: list[str],
        parent_uuid: Optional[str] = None,
        plugin: Optional[str] = None,
    ) -> str:
        return self._inner.record_process(  # type: ignore
            operation, inputs, outputs, parent_uuid, plugin
        )

    def to_json_report(self) -> str:
        return self._inner.to_json_report()  # type: ignore

    def export_parquet(self, path: str) -> None:
        return self._inner.export_parquet(path)  # type: ignore


class WritePlan:
    def __init__(self, inner: Any) -> None:
        self._inner = inner

    def to_json(self) -> str:
        return self._inner.to_json()  # type: ignore

    @staticmethod
    def from_json(json_str: str) -> "WritePlan":
        return WritePlan(core.PyWritePlan.from_json(json_str))


class MatrixEngine:
    def __init__(self, store_path: str) -> None:
        self._inner = core.PyMatrixEngine(store_path)

    def plan_matrix(
        self, index_db_path: str, taxa: list[str], loci: list[tuple[str, int]]
    ) -> WritePlan:
        return WritePlan(self._inner.plan_matrix(index_db_path, taxa, loci))

    def initialize_from_plan(self, plan: WritePlan) -> None:
        return self._inner.initialize_from_plan(plan._inner)  # type: ignore

    def write_taxon_locus(
        self, plan: WritePlan, taxon_id: str, locus_name: str, data: bytes
    ) -> None:
        return self._inner.write_taxon_locus(plan._inner, taxon_id, locus_name, data)  # type: ignore


class PluginRuntime:
    def __init__(self) -> None:
        self._inner = core.PyPluginRuntime()

    def execute_plugin(self, plugin_id: str, table: pa.Table) -> pa.Table:
        sink = io.BytesIO()
        with pa.ipc.new_stream(sink, table.schema) as writer:
            writer.write_table(table)

        batch_bytes = sink.getvalue()
        output_bytes = self._inner.execute_plugin(plugin_id, batch_bytes)

        output_buf = io.BytesIO(output_bytes)
        return pa.ipc.open_stream(output_buf).read_all()

    def get_signatures(self) -> Any:
        return json.loads(self._inner.get_signatures())
