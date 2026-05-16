use arrow::array::{
    ArrayRef, Float64Array, Int64Array, ListBuilder, StringBuilder, TimestampMicrosecondArray,
    StringArray,
};
use arrow::record_batch::RecordBatch;
use parquet::arrow::arrow_writer::ArrowWriter;
use parquet::file::properties::WriterProperties;
use std::fs::File;
use std::sync::Arc;
use crate::{TAXON_ENTITY_SCHEMA, get_current_schema_version};

/// Create a sample Parquet file for TaxonEntity
pub fn create_sample_taxon_parquet(path: &str) -> anyhow::Result<()> {
    let schema = TAXON_ENTITY_SCHEMA.clone();
    
    let uuids = StringArray::from(vec!["550e8400-e29b-41d4-a716-446655440000"]);
    let raw_labels = StringArray::from(vec!["Homo sapiens"]);
    let normalized_labels = StringArray::from(vec![Some("homo sapiens")]);
    let canonical_names = StringArray::from(vec![Some("Homo sapiens")]);
    let ontology_ids = StringArray::from(vec![Some("NCBI:txid9606")]);
    let ranks = StringArray::from(vec![Some("species")]);
    
    let mut lineage_builder = ListBuilder::new(StringBuilder::new());
    lineage_builder.values().append_value("Eukaryota");
    lineage_builder.values().append_value("Metazoa");
    lineage_builder.values().append_value("Chordata");
    lineage_builder.append(true);
    let lineage_array = lineage_builder.finish();

    let lineage_hashes = StringArray::from(vec![Some("deterministic-hash-123")]);
    let source_files = StringArray::from(vec!["sample.fasta"]);
    let source_indices = Int64Array::from(vec![0]);
    let statuses = StringArray::from(vec!["matched"]);
    let confidence_scores = Float64Array::from(vec![1.0]);
    let pipeline_versions = StringArray::from(vec!["0.1.0"]);
    let schema_versions = StringArray::from(vec![get_current_schema_version()]);
    let timestamps = TimestampMicrosecondArray::from_value(1714900000000, 1).with_timezone("UTC");

    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(uuids) as ArrayRef,
            Arc::new(raw_labels) as ArrayRef,
            Arc::new(normalized_labels) as ArrayRef,
            Arc::new(canonical_names) as ArrayRef,
            Arc::new(ontology_ids) as ArrayRef,
            Arc::new(ranks) as ArrayRef,
            Arc::new(lineage_array) as ArrayRef,
            Arc::new(lineage_hashes) as ArrayRef,
            Arc::new(source_files) as ArrayRef,
            Arc::new(source_indices) as ArrayRef,
            Arc::new(statuses) as ArrayRef,
            Arc::new(confidence_scores) as ArrayRef,
            Arc::new(pipeline_versions) as ArrayRef,
            Arc::new(schema_versions) as ArrayRef,
            Arc::new(timestamps) as ArrayRef,
        ],
    )?;

    let file = File::create(path)?;
    let props = WriterProperties::builder().build();
    let mut writer = ArrowWriter::try_new(file, schema, Some(props))?;

    writer.write(&batch)?;
    writer.close()?;

    Ok(())
}
