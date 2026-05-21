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
use regex::Regex;

use superalign_schemas::MATCH_PROVENANCE_SCHEMA;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Candidate {
    pub name: String,
    pub ontology_id: String,
    pub score: f64,
    pub rank: String,
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
                lineage_path TEXT,
                synonyms TEXT
            )",
            [],
        )?;
        // Table for Accession to TaxID mapping
        conn.execute(
            "CREATE TABLE IF NOT EXISTS accessions (
                accession_id TEXT PRIMARY KEY,
                ontology_id TEXT NOT NULL REFERENCES ontology(ontology_id)
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
            ('NCBI:10090', 'Mus musculus', 'species', 'Eukaryota|Metazoa|Chordata|Muridae|Mus|musculus', '[\"house mouse\", \"mouse\"]'),
            ('NCBI:6978', 'Periplaneta americana', 'species', 'Eukaryota|Metazoa|Arthropoda|Blattidae|Periplaneta|americana', '[\"American cockroach\"]')",
            [],
        )?;
        self.conn.execute(
            "INSERT INTO accessions (accession_id, ontology_id) VALUES 
            ('NC_000001', 'NCBI:9606'),
            ('NC_000067', 'NCBI:10090')",
            [],
        )?;
        Ok(())
    }

    pub fn find_by_id(&self, id: &str) -> Result<Option<(String, String, String, String)>> {
        let mut stmt = self.conn.prepare("SELECT ontology_id, name, lineage_path, rank FROM ontology WHERE ontology_id = ?")?;
        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            Ok(Some((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)))
        } else {
            Ok(None)
        }
    }

    pub fn find_by_accession(&self, acc: &str) -> Result<Option<(String, String, String, String)>> {
        let mut stmt = self.conn.prepare("SELECT o.ontology_id, o.name, o.lineage_path, o.rank 
                                          FROM ontology o JOIN accessions a ON o.ontology_id = a.ontology_id 
                                          WHERE a.accession_id = ?")?;
        let mut rows = stmt.query(params![acc])?;
        if let Some(row) = rows.next()? {
            Ok(Some((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)))
        } else {
            Ok(None)
        }
    }

    pub fn find_exact_match(&self, label: &str) -> Result<Option<(String, String, String, String)>> {
        let mut stmt = self.conn.prepare("SELECT ontology_id, name, lineage_path, rank FROM ontology WHERE name = ? OR synonyms LIKE ? LIMIT 1")?;
        let synonym_query = format!("%\"{}\"%", label);
        let mut rows = stmt.query(params![label, synonym_query])?;
        
        if let Some(row) = rows.next()? {
            Ok(Some((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)))
        } else {
            Ok(None)
        }
    }

    pub fn get_all_candidates(&self) -> Result<Vec<Candidate>> {
        let mut stmt = self.conn.prepare("SELECT ontology_id, name, rank FROM ontology")?;
        let rows = stmt.query_map([], |row| {
            Ok(Candidate {
                ontology_id: row.get(0)?,
                name: row.get(1)?,
                rank: row.get(2)?,
                score: 0.0,
            })
        })?;
        
        let mut results = Vec::new();
        for r in rows {
            results.push(r?);
        }
        Ok(results)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TieBreakingStrategy {
    LowTaxId,
    HighRank,
}

pub struct Reconciler {
    db: TaxonomyDb,
    threshold: f64,
    taxid_regex: Regex,
    accession_regex: Regex,
    tie_breaker: TieBreakingStrategy,
}

impl Reconciler {
    pub fn new(db: TaxonomyDb, threshold: f64) -> Self {
        let taxid_regex = Regex::new(r"(?i)(?:taxid:|ox:)(\d+)").unwrap();
        let accession_regex = Regex::new(r"(?i)([A-Z]{1,2}_\d{6,})").unwrap(); // Simplified accession regex
        Self { 
            db, 
            threshold, 
            taxid_regex,
            accession_regex,
            tie_breaker: TieBreakingStrategy::LowTaxId,
        }
    }

    pub fn with_tie_breaker(mut self, strategy: TieBreakingStrategy) -> Self {
        self.tie_breaker = strategy;
        self
    }

    fn extract_taxid(&self, label: &str) -> Option<String> {
        self.taxid_regex.captures(label).map(|cap| format!("NCBI:{}", &cap[1]))
    }

    fn extract_accession(&self, label: &str) -> Option<String> {
        self.accession_regex.captures(label).map(|cap| cap[1].to_uppercase())
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

        let mut canonical_names = StringBuilder::new();
        let mut ontology_ids = StringBuilder::new();
        let mut lineage_arrays = ListBuilder::new(StringBuilder::new());
        let mut confidence_scores = Float64Builder::new();
        let mut statuses = StringBuilder::new();

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

        let all_ontology = self.db.get_all_candidates()?;

        for i in 0..row_count {
            let label = raw_labels.value(i);
            let entity_uuid = entity_uuids.value(i);

            // PRIORITY HIERARCHY
            let mut match_result = None;
            let mut current_algo = "NONE";

            // 1. TaxID
            if let Some(taxid) = self.extract_taxid(label) {
                if let Some(meta) = self.db.find_by_id(&taxid)? {
                    match_result = Some((meta.0, meta.1, meta.2, meta.3, 1.0));
                    current_algo = "NCBI_TAXID_EXTRACT";
                }
            }

            // 2. Accession
            if match_result.is_none() {
                if let Some(acc) = self.extract_accession(label) {
                    if let Some(meta) = self.db.find_by_accession(&acc)? {
                        match_result = Some((meta.0, meta.1, meta.2, meta.3, 1.0));
                        current_algo = "ACCESSION_GROUNDING";
                    }
                }
            }

            // 3. Exact Name/Synonym
            if match_result.is_none() {
                if let Some(meta) = self.db.find_exact_match(label)? {
                    match_result = Some((meta.0, meta.1, meta.2, meta.3, 1.0));
                    current_algo = "EXACT_ONTOLOGY_MATCH";
                }
            }

            // 4. Fuzzy Fallback
            let mut candidates = Vec::new();
            if match_result.is_none() {
                for cand in &all_ontology {
                    let score = jaro_winkler(label, &cand.name);
                    if score >= self.threshold {
                        let mut c = cand.clone();
                        c.score = score;
                        candidates.push(c);
                    }
                }

                // Deterministic Tie-Breaking
                candidates.sort_by(|a, b| {
                    b.score.partial_cmp(&a.score).unwrap()
                        .then_with(|| {
                            // Rank priority: species > genus etc (simplified)
                            let r_a = if a.rank == "species" { 0 } else { 1 };
                            let r_b = if b.rank == "species" { 0 } else { 1 };
                            r_a.cmp(&r_b)
                        })
                        .then_with(|| a.ontology_id.cmp(&b.ontology_id))
                });

                if let Some(best) = candidates.first() {
                    // Check for true ambiguity: multiple candidates with same top score and same rank
                    let top_score = best.score;
                    let ambiguous = candidates.iter().filter(|c| c.score == top_score).count() > 1;
                    
                    if ambiguous {
                        current_algo = "AMBIGUOUS_FUZZY";
                    } else {
                        current_algo = "JARO_WINKLER_FALLBACK";
                    }
                    
                    // We still pick the 'best' one via tie-breaking for determinism, 
                    // but status will reflect ambiguity.
                    if let Some(meta) = self.db.find_by_id(&best.ontology_id)? {
                        match_result = Some((meta.0, meta.1, meta.2, meta.3, best.score));
                    }
                }
            }

            // Persistence
            ontology_sources.append_value("NCBI_Taxonomy_Workstation_Local");
            normalization_steps_list.append(true);
            execution_hashes.append_value("v1_hierarchical_reconciler");

            if let Some((oid, name, path, _rank, score)) = match_result {
                canonical_names.append_value(&name);
                ontology_ids.append_value(&oid);
                
                let parts: Vec<&str> = path.split('|').collect();
                let lineage_builder = lineage_arrays.values();
                for p in parts {
                    lineage_builder.append_value(p);
                }
                lineage_arrays.append(true);

                confidence_scores.append_value(score);
                
                if current_algo == "AMBIGUOUS_FUZZY" {
                    statuses.append_value("ISOLATED_PENDING_REVIEW");
                } else {
                    statuses.append_value("RECONCILED");
                }

                prov_uuids.append_value(Uuid::new_v4().to_string());
                prov_entity_uuids.append_value(entity_uuid);
                algorithms.append_value(current_algo);
                thresholds.append_value(self.threshold);
                distances.append_value(score);
                selected_candidates.append_value(&name);
                
                let competing_json = serde_json::to_string(&candidates).unwrap_or_else(|_| "[]".to_string());
                competing_candidates.append_value(&competing_json);
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
    fn test_hierarchical_reconciliation() -> Result<()> {
        let db = TaxonomyDb::new_in_memory()?;
        db.load_sample_ontology()?;
        let reconciler = Reconciler::new(db, 0.8);
        
        let batch = RecordBatch::try_new(
            Arc::new(arrow::datatypes::Schema::new(vec![
                arrow::datatypes::Field::new("raw_label", arrow::datatypes::DataType::Utf8, false),
                arrow::datatypes::Field::new("entity_uuid", arrow::datatypes::DataType::Utf8, false),
            ])),
            vec![
                Arc::new(StringArray::from(vec![
                    "Homo sapiens [taxid:9606]", // TaxID match
                    "NC_000067.1",               // Accession match
                    "human",                     // Synonym match
                    "Mus muscul"                 // Fuzzy match
                ])) as ArrayRef,
                Arc::new(StringArray::from(vec!["u1", "u2", "u3", "u4"])) as ArrayRef,
            ]
        )?;
        
        let (reconciled, provenance) = reconciler.reconcile_batch(&batch)?;
        
        let names = reconciled.column_by_name("canonical_name").unwrap().as_any().downcast_ref::<StringArray>().unwrap();
        let algos = provenance.column_by_name("matching_algorithm").unwrap().as_any().downcast_ref::<StringArray>().unwrap();
        let statuses = reconciled.column_by_name("reconciliation_status").unwrap().as_any().downcast_ref::<StringArray>().unwrap();

        assert_eq!(names.value(0), "Homo sapiens");
        assert_eq!(algos.value(0), "NCBI_TAXID_EXTRACT");
        
        assert_eq!(names.value(1), "Mus musculus");
        assert_eq!(algos.value(1), "ACCESSION_GROUNDING");

        assert_eq!(names.value(2), "Homo sapiens");
        assert_eq!(algos.value(2), "EXACT_ONTOLOGY_MATCH");

        assert_eq!(names.value(3), "Mus musculus");
        assert_eq!(algos.value(3), "JARO_WINKLER_FALLBACK");
        assert_eq!(statuses.value(3), "RECONCILED");

        Ok(())
    }

    #[test]
    fn test_ambiguity_flagging() -> Result<()> {
        let db = TaxonomyDb::new_in_memory()?;
        // Add two species with very similar names
        db.conn.execute("INSERT INTO ontology VALUES ('NCBI:1', 'Similar Specimen A', 'species', 'root', '[]')", [])?;
        db.conn.execute("INSERT INTO ontology VALUES ('NCBI:2', 'Similar Specimen B', 'species', 'root', '[]')", [])?;
        
        let reconciler = Reconciler::new(db, 0.5);
        let batch = RecordBatch::try_new(
            Arc::new(arrow::datatypes::Schema::new(vec![
                arrow::datatypes::Field::new("raw_label", arrow::datatypes::DataType::Utf8, false),
                arrow::datatypes::Field::new("entity_uuid", arrow::datatypes::DataType::Utf8, false),
            ])),
            vec![
                Arc::new(StringArray::from(vec!["Similar Specimen"])) as ArrayRef,
                Arc::new(StringArray::from(vec!["u1"])) as ArrayRef,
            ]
        )?;

        let (reconciled, provenance) = reconciler.reconcile_batch(&batch)?;
        let statuses = reconciled.column_by_name("reconciliation_status").unwrap().as_any().downcast_ref::<StringArray>().unwrap();
        let algos = provenance.column_by_name("matching_algorithm").unwrap().as_any().downcast_ref::<StringArray>().unwrap();
        
        assert_eq!(statuses.value(0), "ISOLATED_PENDING_REVIEW");
        assert_eq!(algos.value(0), "AMBIGUOUS_FUZZY");
        
        Ok(())
    }
}
