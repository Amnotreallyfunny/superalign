use arrow::datatypes::{DataType, Field, Schema, TimeUnit};
use lazy_static::lazy_static;
use std::sync::Arc;

#[cfg(feature = "parquet")]
pub mod sample_data;
pub mod contracts;
pub mod invariants;

lazy_static! {
    /// Canonical schema for TaxonEntity
    pub static ref TAXON_ENTITY_SCHEMA: Arc<Schema> = Arc::new(Schema::new(vec![
        Field::new("entity_uuid", DataType::Utf8, false),
        Field::new("raw_label", DataType::Utf8, false),
        Field::new("normalized_label", DataType::Utf8, true),
        Field::new("canonical_name", DataType::Utf8, true),
        Field::new("ontology_id", DataType::Utf8, true),
        Field::new("taxonomy_rank", DataType::Utf8, true),
        Field::new("lineage_array", DataType::List(Arc::new(Field::new("item", DataType::Utf8, true))), true),
        Field::new("lineage_hash", DataType::Utf8, true),
        Field::new("source_file", DataType::Utf8, false),
        Field::new("source_record_index", DataType::Int64, false),
        Field::new("reconciliation_status", DataType::Utf8, false),
        Field::new("confidence_score", DataType::Float64, false),
        Field::new("pipeline_version", DataType::Utf8, false),
        Field::new("schema_version", DataType::Utf8, false),
        Field::new("timestamp", DataType::Timestamp(TimeUnit::Microsecond, Some("UTC".into())), false),
    ]));

    /// Canonical schema for MatchProvenance
    pub static ref MATCH_PROVENANCE_SCHEMA: Arc<Schema> = Arc::new(Schema::new(vec![
        Field::new("provenance_uuid", DataType::Utf8, false),
        Field::new("entity_uuid", DataType::Utf8, false),
        Field::new("matching_algorithm", DataType::Utf8, false),
        Field::new("threshold_used", DataType::Float64, false),
        Field::new("computed_distance", DataType::Float64, false),
        Field::new("competing_candidates", DataType::Utf8, true), // JSON string for flexibility
        Field::new("selected_candidate", DataType::Utf8, true),
        Field::new("ontology_source", DataType::Utf8, false),
        Field::new("normalization_steps", DataType::List(Arc::new(Field::new("item", DataType::Utf8, true))), true),
        Field::new("reconciliation_timestamp", DataType::Timestamp(TimeUnit::Microsecond, Some("UTC".into())), false),
        Field::new("execution_hash", DataType::Utf8, false),
    ]));

    /// Canonical schema for Sequence Metadata
    pub static ref SEQUENCE_METADATA_SCHEMA: Arc<Schema> = Arc::new(Schema::new(vec![
        Field::new("entity_uuid", DataType::Utf8, false),
        Field::new("sequence_hash", DataType::Utf8, false),
        Field::new("sequence_len", DataType::Int64, false),
        Field::new("is_valid", DataType::Boolean, false),
        Field::new("composition", DataType::Utf8, true), // JSON string e.g. {"A": 10, "C": 5...}
    ]));

    /// Canonical schema for Process Provenance (Transformation DAG nodes)
    pub static ref PROCESS_PROVENANCE_SCHEMA: Arc<Schema> = Arc::new(Schema::new(vec![
        Field::new("process_uuid", DataType::Utf8, false),
        Field::new("parent_process_uuid", DataType::Utf8, true),
        Field::new("operation_name", DataType::Utf8, false),
        Field::new("input_hashes", DataType::List(Arc::new(Field::new("item", DataType::Utf8, true))), false),
        Field::new("output_hashes", DataType::List(Arc::new(Field::new("item", DataType::Utf8, true))), false),
        Field::new("execution_context_hash", DataType::Utf8, false),
        Field::new("plugin_id", DataType::Utf8, true),
        Field::new("pipeline_version", DataType::Utf8, false),
        Field::new("timestamp", DataType::Timestamp(TimeUnit::Microsecond, Some("UTC".into())), false),
    ]));
}

/// Utility to get current schema version
pub fn get_current_schema_version() -> &'static str {
    "1.0.0"
}

/// Deterministic hashing for execution blocks
pub fn compute_execution_hash(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

#[cfg(test) ]
mod tests {
    use super::*;

    #[test]
    fn test_taxon_entity_schema_fields() {
        assert_eq!(TAXON_ENTITY_SCHEMA.fields().len(), 15);
        assert!(TAXON_ENTITY_SCHEMA.column_with_name("entity_uuid").is_some());
    }

    #[test]
    fn test_provenance_schema_fields() {
        assert_eq!(MATCH_PROVENANCE_SCHEMA.fields().len(), 11);
    }
}
