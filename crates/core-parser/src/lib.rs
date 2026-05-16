use anyhow::{Context, Result};
use arrow::array::{
    ArrayRef, BooleanBuilder, Float64Builder, Int64Builder, StringBuilder, TimestampMicrosecondBuilder,
    ListBuilder,
};
use arrow::record_batch::RecordBatch;
use needletail::parse_fastx_file;
use sha2::{Digest, Sha256};
use std::path::Path;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

use superalign_schemas::{TAXON_ENTITY_SCHEMA, SEQUENCE_METADATA_SCHEMA, get_current_schema_version};

pub struct FastaParseResult {
    pub taxon_entities: RecordBatch,
    pub sequence_metadata: RecordBatch,
}

pub struct FastaParser {
    batch_size: usize,
    pipeline_version: String,
}

impl FastaParser {
    pub fn new(batch_size: usize, pipeline_version: &str) -> Self {
        Self {
            batch_size,
            pipeline_version: pipeline_version.to_string(),
        }
    }

    pub fn parse<P: AsRef<Path>>(&self, path: P) -> Result<impl Iterator<Item = Result<FastaParseResult>>> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        let mut reader = parse_fastx_file(&path)
            .with_context(|| format!("Failed to open fastx file: {}", path_str))?;
        
        let batch_size = self.batch_size;
        let pipeline_version = self.pipeline_version.clone();
        let schema_version = get_current_schema_version().to_string();

        Ok(std::iter::from_fn(move || {
            // Entity Builders
            let mut entity_uuids = StringBuilder::new();
            let mut raw_labels = StringBuilder::new();
            let mut normalized_labels = StringBuilder::new();
            let mut canonical_names = StringBuilder::new();
            let mut ontology_ids = StringBuilder::new();
            let mut taxonomy_ranks = StringBuilder::new();
            let mut lineage_arrays = ListBuilder::new(StringBuilder::new());
            let mut lineage_hashes = StringBuilder::new();
            let mut source_files = StringBuilder::new();
            let mut source_indices = Int64Builder::new();
            let mut statuses = StringBuilder::new();
            let mut confidence_scores = Float64Builder::new();
            let mut pipeline_versions = StringBuilder::new();
            let mut schema_versions = StringBuilder::new();
            let mut timestamps = TimestampMicrosecondBuilder::new().with_timezone("UTC");

            // Metadata Builders
            let mut meta_uuids = StringBuilder::new();
            let mut sequence_hashes = StringBuilder::new();
            let mut sequence_lens = Int64Builder::new();
            let mut is_valids = BooleanBuilder::new();
            let mut compositions = StringBuilder::new();

            let mut count = 0;

            while count < batch_size {
                if let Some(record) = reader.next() {
                    let rec = match record {
                        Ok(r) => r,
                        Err(e) => return Some(Err(anyhow::anyhow!("Fastx parse error: {}", e))),
                    };

                    let uuid = Uuid::new_v4().to_string();
                    let header = String::from_utf8_lossy(rec.id()).trim().to_string();
                    
                    // Compute SHA-256
                    let mut hasher = Sha256::new();
                    hasher.update(rec.seq());
                    let hash = hex::encode(hasher.finalize());

                    // Build Entity Row
                    entity_uuids.append_value(&uuid);
                    raw_labels.append_value(&header);
                    normalized_labels.append_null();
                    canonical_names.append_null();
                    ontology_ids.append_null();
                    taxonomy_ranks.append_null();
                    lineage_arrays.append(false); // Null list
                    lineage_hashes.append_null();
                    source_files.append_value(&path_str);
                    source_indices.append_value(count as i64); // TODO: Global counter
                    statuses.append_value("UNRECONCILED");
                    confidence_scores.append_value(0.0);
                    pipeline_versions.append_value(&pipeline_version);
                    schema_versions.append_value(&schema_version);
                    timestamps.append_value(Utc::now().timestamp_micros());

                    // Build Metadata Row
                    meta_uuids.append_value(&uuid);
                    sequence_hashes.append_value(&hash);
                    sequence_lens.append_value(rec.seq().len() as i64);
                    is_valids.append_value(true);
                    compositions.append_null(); // Future: JSON composition

                    count += 1;
                } else {
                    break;
                }
            }

            if count == 0 {
                return None;
            }

            let entity_batch = RecordBatch::try_new(
                TAXON_ENTITY_SCHEMA.clone(),
                vec![
                    Arc::new(entity_uuids.finish()) as ArrayRef,
                    Arc::new(raw_labels.finish()) as ArrayRef,
                    Arc::new(normalized_labels.finish()) as ArrayRef,
                    Arc::new(canonical_names.finish()) as ArrayRef,
                    Arc::new(ontology_ids.finish()) as ArrayRef,
                    Arc::new(taxonomy_ranks.finish()) as ArrayRef,
                    Arc::new(lineage_arrays.finish()) as ArrayRef,
                    Arc::new(lineage_hashes.finish()) as ArrayRef,
                    Arc::new(source_files.finish()) as ArrayRef,
                    Arc::new(source_indices.finish()) as ArrayRef,
                    Arc::new(statuses.finish()) as ArrayRef,
                    Arc::new(confidence_scores.finish()) as ArrayRef,
                    Arc::new(pipeline_versions.finish()) as ArrayRef,
                    Arc::new(schema_versions.finish()) as ArrayRef,
                    Arc::new(timestamps.finish()) as ArrayRef,
                ],
            );

            let meta_batch = RecordBatch::try_new(
                SEQUENCE_METADATA_SCHEMA.clone(),
                vec![
                    Arc::new(meta_uuids.finish()) as ArrayRef,
                    Arc::new(sequence_hashes.finish()) as ArrayRef,
                    Arc::new(sequence_lens.finish()) as ArrayRef,
                    Arc::new(is_valids.finish()) as ArrayRef,
                    Arc::new(compositions.finish()) as ArrayRef,
                ],
            );

            match (entity_batch, meta_batch) {
                (Ok(e), Ok(m)) => Some(Ok(FastaParseResult { taxon_entities: e, sequence_metadata: m })),
                (Err(e), _) => Some(Err(e.into())),
                (_, Err(e)) => Some(Err(e.into())),
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_fasta_parsing() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, ">seq1\nACGT\n>seq2\nTGCA")?;
        
        let parser = FastaParser::new(10, "0.1.0");
        let results: Vec<_> = parser.parse(file.path())?.collect();
        
        assert_eq!(results.len(), 1);
        let res = results[0].as_ref().unwrap();
        
        assert_eq!(res.taxon_entities.num_rows(), 2);
        assert_eq!(res.sequence_metadata.num_rows(), 2);
        
        Ok(())
    }
}
