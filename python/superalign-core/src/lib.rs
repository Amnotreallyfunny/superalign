use pyo3::prelude::*;
use pyo3::types::PyBytes;
use arrow::record_batch::RecordBatch;
use arrow::ipc::writer::StreamWriter;
use arrow::ipc::reader::StreamReader;
use std::io::Cursor;
use core_parser::FastaParser;
use taxonomy_engine::{TaxonomyDb, Reconciler};
use matrix_engine::MatrixEngine;
use provenance_core::ProvenanceManager;
use superalign_schemas::{get_current_schema_version};

#[pyclass]
struct PyMatrixEngine {
    inner: MatrixEngine,
}

#[pymethods]
impl PyMatrixEngine {
    #[new]
    fn new(store_path: String) -> Self {
        Self {
            inner: MatrixEngine::new(&store_path),
        }
    }

    fn create_matrix(&self, num_taxa: u64, total_length: u64) -> PyResult<()> {
        self.inner.create_matrix(num_taxa, total_length)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    fn write_chunk(&self, taxon_index: u64, start_pos: u64, data: Vec<u8>) -> PyResult<()> {
        self.inner.write_chunk(taxon_index, start_pos, &data)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
}

#[pyclass]
struct PyProvenanceManager {
    inner: ProvenanceManager,
}

#[pymethods]
impl PyProvenanceManager {
    #[new]
    fn new(pipeline_version: String) -> Self {
        Self {
            inner: ProvenanceManager::new(&pipeline_version),
        }
    }

    fn record_process(
        &mut self,
        operation: String,
        inputs: Vec<String>,
        outputs: Vec<String>,
        parent_uuid: Option<String>,
        plugin: Option<String>,
    ) -> String {
        self.inner.record_process(&operation, inputs, outputs, parent_uuid, plugin)
    }

    fn to_json_report(&self) -> PyResult<String> {
        self.inner.to_json_report().map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    fn export_parquet(&self, path: String) -> PyResult<()> {
        self.inner.export_parquet(&path).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
}

#[pyfunction]
fn schema_version() -> String {
    get_current_schema_version().to_string()
}

#[pyfunction]
fn parse_fasta(py: Python<'_>, path: String, batch_size: usize) -> PyResult<PyObject> {
    let parser = FastaParser::new(batch_size, "0.1.0");
    let results = parser.parse(&path).map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

    let mut py_results = Vec::new();

    for res in results {
        let res = res.map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        
        let entity_bytes = serialize_batch(&res.taxon_entities)?;
        let meta_bytes = serialize_batch(&res.sequence_metadata)?;
        
        let dict = pyo3::types::PyDict::new(py);
        dict.set_item("taxon_entities", PyBytes::new(py, &entity_bytes))?;
        dict.set_item("sequence_metadata", PyBytes::new(py, &meta_bytes))?;
        
        py_results.push(dict.to_object(py));
    }

    Ok(py_results.to_object(py))
}

#[pyfunction]
fn reconcile_batch(py: Python<'_>, batch_bytes: Vec<u8>, threshold: f64) -> PyResult<PyObject> {
    let cursor = Cursor::new(batch_bytes);
    let mut reader = StreamReader::try_new(cursor, None)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    
    let batch = reader.next()
        .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Empty batch"))?
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

    let db = TaxonomyDb::new_in_memory()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    db.load_sample_ontology()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    
    let reconciler = Reconciler::new(db, threshold);
    let (reconciled, provenance) = reconciler.reconcile_batch(&batch)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

    let reconciled_bytes = serialize_batch(&reconciled)?;
    let provenance_bytes = serialize_batch(&provenance)?;

    let dict = pyo3::types::PyDict::new(py);
    dict.set_item("reconciled", PyBytes::new(py, &reconciled_bytes))?;
    dict.set_item("provenance", PyBytes::new(py, &provenance_bytes))?;

    Ok(dict.to_object(py))
}

fn serialize_batch(batch: &RecordBatch) -> PyResult<Vec<u8>> {
    let mut buffer = Vec::new();
    {
        let mut writer = StreamWriter::try_new(&mut buffer, &batch.schema())
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        writer.write(batch)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        writer.finish()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    }
    Ok(buffer)
}

#[pymodule]
fn core(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(schema_version, m)?)?;
    m.add_function(wrap_pyfunction!(parse_fasta, m)?)?;
    m.add_function(wrap_pyfunction!(reconcile_batch, m)?)?;
    m.add_class::<PyProvenanceManager>()?;
    m.add_class::<PyMatrixEngine>()?;
    Ok(())
}
