use anyhow::{Context, Result};
use arrow::array::{
    Array, ArrayRef, Float64Builder, ListBuilder, StringBuilder, StringArray, TimestampMicrosecondBuilder,
};
use arrow::record_batch::RecordBatch;
use duckdb::{params, Connection};
use strsim::jaro_winkler;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use serde::{Serialize, Deserialize};

use superalign_schemas::{TAXON_ENTITY_SCHEMA, MATCH_PROVENANCE_SCHEMA};

#[derive(Serialize, Deserialize, Debug)]
pub struct Candidate {
    pub name: String,
    pub ontology_id: String,
    pub score: f64,
}

pub struct TaxonomyDb {
    conn: Connection,
}

impl TaxonomyDb {
    pub fn new_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        Self::initialize_schema(&conn)?;
        Ok(Self { conn })
    }

    fn initialize_schema(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS ontology (
                ontology_id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                rank TEXT,
                lineage_path TEXT, -- Materialized path: Eukaryota|Chordata|...
                synonyms TEXT -- JSON array of synonyms
            )",
            [],
        )?;
        conn.execute("CREATE INDEX IF NOT EXISTS idx_ontology_name ON ontology(name)", [])?;
        Ok(())
    }

    pub fn load_sample_ontology(&self) -> Result<()> {
        self.conn.execute(
            "INSERT INTO ontology (ontology_id, name, rank, lineage_path, synonyms) VALUES 
            ('NCBI:9606', 'Homo sapiens', 'species', 'Eukaryota|Metazoa|Chordata|Hominidae|Homo|sapiens', '[\"human\", \"man\"]'),
            ('NCBI:10090', 'Mus musculus', 'species', 'Eukaryota|Metazoa|Chordata|Muridae|Mus|musculus', '[\"house mouse\", \"mouse\"]')",
            [],
        )?;
        Ok(())
    }

    pub fn find_exact_match(&self, label: &str) -> Result<Option<(String, String, String)>> {
        let mut stmt = self.conn.prepare("SELECT ontology_id, name, lineage_path FROM ontology WHERE name = ? OR synonyms LIKE ? LIMIT 1")?;
        let synonym_query = format!("%\"{}\"%", label);
        let mut rows = stmt.query(params![label, synonym_query])?;
        
        if let Some(row) = rows.next()? {
            Ok(Some((row.get(0)?, row.get(1)?, row.get(2)?)))
        } else {
            Ok(None)
        }
    }

    pub fn get_all_names(&self) -> Result<Vec<(String, String, String)>> {
        let mut stmt = self.conn.prepare("SELECT ontology_id, name, lineage_path FROM ontology")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;
        
        let mut results = Vec::new();
        for r in rows {
            results.push(r?);
        }
        Ok(results)
    }
}

pub struct Reconciler {
    db: TaxonomyDb,
    threshold: f64,
}

impl Reconciler {
    pub fn new(db: TaxonomyDb, threshold: f64) -> Self {
        Self { db, threshold }
    }

    pub fn reconcile_batch(&self, batch: &RecordBatch) -> Result<(RecordBatch, RecordBatch)> {
        let raw_labels = batch
            .column_by_name("raw_label")
            .context("Missing raw_label column")?
            .as_any()
            .downcast_ref::<StringArray>()
            .context("raw_label is not a string array")?;

        let entity_uuids = batch
            .column_by_name("entity_uuid")
            .context("Missing entity_uuid column")?
            .as_any()
            .downcast_ref::<StringArray>()
            .context("entity_uuid is not a string array")?;

        let row_count = batch.num_rows();

        // Builders for updated TaxonEntity
        let mut canonical_names = StringBuilder::new();
        let mut ontology_ids = StringBuilder::new();
        let mut lineage_arrays = ListBuilder::new(StringBuilder::new());
        let mut confidence_scores = Float64Builder::new();
        let mut statuses = StringBuilder::new();

        // Builders for MatchProvenance
        let mut prov_uuids = StringBuilder::new();
        let mut prov_entity_uuids = StringBuilder::new();
        let mut algorithms = StringBuilder::new();
        let mut thresholds = Float64Builder::new();
        let mut distances = Float64Builder::new();
        let mut selected_candidates = StringBuilder::new();
        let mut competing_candidates = StringBuilder::new();
        let mut ontology_sources = StringBuilder::new();
        let mut normalization_steps_list = ListBuilder::new(StringBuilder::new());
        let mut prov_timestamps = TimestampMicrosecondBuilder::new().with_timezone("UTC");
        let mut execution_hashes = StringBuilder::new();

        let all_ontology = self.db.get_all_names()?;

        for i in 0..row_count {
            let label = raw_labels.value(i);
            let entity_uuid = entity_uuids.value(i);

            // Try exact match first
            let match_result = if let Some(exact) = self.db.find_exact_match(label)? {
                Some((exact.0, exact.1, exact.2, 1.0, "EXACT"))
            } else {
                // Fuzzy matching
                let mut best_score = 0.0;
                let mut best_match = None;
                let mut candidates = Vec::new();

                for (oid, name, path) in &all_ontology {
                    let score = jaro_winkler(label, name);
                    if score >= self.threshold {
                        candidates.push(Candidate {
                            name: name.clone(),
                            ontology_id: oid.clone(),
                            score,
                        });
                        if score > best_score {
                            best_score = score;
                            best_match = Some((oid.clone(), name.clone(), path.clone(), score, "JARO_WINKLER"));
                        }
                    }
                }
                
                candidates.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
                best_match.map(|m| (m.0, m.1, m.2, m.3, m.4))
            };

            // Fill all builders for each row
            ontology_sources.append_value("NCBI_Taxonomy_Sample");
            normalization_steps_list.append(true); // Empty list
            execution_hashes.append_value("fixed_execution_hash");

            if let Some((oid, name, path, score, algo)) = match_result {
                canonical_names.append_value(&name);
                ontology_ids.append_value(&oid);
                
                let parts: Vec<&str> = path.split('|').collect();
                let lineage_builder = lineage_arrays.values();
                for p in parts {
                    lineage_builder.append_value(p);
                }
                lineage_arrays.append(true);

                confidence_scores.append_value(score);
                statuses.append_value("RECONCILED");

                // Provenance
                prov_uuids.append_value(Uuid::new_v4().to_string());
                prov_entity_uuids.append_value(entity_uuid);
                algorithms.append_value(algo);
                thresholds.append_value(self.threshold);
                distances.append_value(score);
                selected_candidates.append_value(&name);
                competing_candidates.append_value("[]");
                prov_timestamps.append_value(Utc::now().timestamp_micros());
            } else {
                canonical_names.append_null();
                ontology_ids.append_null();
                lineage_arrays.append(false);
                confidence_scores.append_value(0.0);
                statuses.append_value("UNMATCHED");

                prov_uuids.append_value(Uuid::new_v4().to_string());
                prov_entity_uuids.append_value(entity_uuid);
                algorithms.append_value("NONE");
                thresholds.append_value(self.threshold);
                distances.append_value(0.0);
                selected_candidates.append_null();
                competing_candidates.append_value("[]");
                prov_timestamps.append_value(Utc::now().timestamp_micros());
            }
        }

        let reconciled_batch = RecordBatch::try_new(
            Arc::new(arrow::datatypes::Schema::new(vec![
                arrow::datatypes::Field::new("canonical_name", arrow::datatypes::DataType::Utf8, true),
                arrow::datatypes::Field::new("ontology_id", arrow::datatypes::DataType::Utf8, true),
                arrow::datatypes::Field::new("confidence_score", arrow::datatypes::DataType::Float64, false),
                arrow::datatypes::Field::new("reconciliation_status", arrow::datatypes::DataType::Utf8, false),
            ])),
            vec![
                Arc::new(canonical_names.finish()) as ArrayRef,
                Arc::new(ontology_ids.finish()) as ArrayRef,
                Arc::new(confidence_scores.finish()) as ArrayRef,
                Arc::new(statuses.finish()) as ArrayRef,
            ]
        )?;

        let provenance_batch = RecordBatch::try_new(
            MATCH_PROVENANCE_SCHEMA.clone(),
            vec![
                Arc::new(prov_uuids.finish()) as ArrayRef,
                Arc::new(prov_entity_uuids.finish()) as ArrayRef,
                Arc::new(algorithms.finish()) as ArrayRef,
                Arc::new(thresholds.finish()) as ArrayRef,
                Arc::new(distances.finish()) as ArrayRef,
                Arc::new(competing_candidates.finish()) as ArrayRef,
                Arc::new(selected_candidates.finish()) as ArrayRef,
                Arc::new(ontology_sources.finish()) as ArrayRef,
                Arc::new(normalization_steps_list.finish()) as ArrayRef,
                Arc::new(prov_timestamps.finish()) as ArrayRef,
                Arc::new(execution_hashes.finish()) as ArrayRef,
            ]
        )?;

        Ok((reconciled_batch, provenance_batch))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow::array::StringArray;

    #[test]
    fn test_reconciliation() -> Result<()> {
        let db = TaxonomyDb::new_in_memory()?;
        db.load_sample_ontology()?;
        
        let reconciler = Reconciler::new(db, 0.8);
        
        let batch = RecordBatch::try_new(
            Arc::new(arrow::datatypes::Schema::new(vec![
                arrow::datatypes::Field::new("raw_label", arrow::datatypes::DataType::Utf8, false),
                arrow::datatypes::Field::new("entity_uuid", arrow::datatypes::DataType::Utf8, false),
            ])),
            vec![
                Arc::new(StringArray::from(vec!["Homo sapiens", "mouse", "unknown"])) as ArrayRef,
                Arc::new(StringArray::from(vec!["uuid1", "uuid2", "uuid3"])) as ArrayRef,
            ]
        )?;
        
        let (reconciled, provenance) = reconciler.reconcile_batch(&batch)?;
        
        assert_eq!(reconciled.num_rows(), 3);
        assert_eq!(provenance.num_rows(), 3);
        
        Ok(())
    }
}
