use anyhow::{Result, anyhow};
use arrow::record_batch::RecordBatch;
use arrow::array::{StringArray, Float64Array};
use std::sync::Arc;

pub struct InvariantEngine;

impl InvariantEngine {
    /// Validates basic taxonomic invariants in a reconciled batch.
    pub fn validate_reconciliation_invariants(batch: &RecordBatch) -> Result<()> {
        let statuses = batch.column_by_name("reconciliation_status")
            .ok_or_else(|| anyhow!("Missing reconciliation_status column"))?
            .as_any().downcast_ref::<StringArray>().unwrap();

        let scores = batch.column_by_name("confidence_score")
            .ok_or_else(|| anyhow!("Missing confidence_score column"))?
            .as_any().downcast_ref::<Float64Array>().unwrap();

        for i in 0..batch.num_rows() {
            let status = statuses.value(i);
            let score = scores.value(i);

            // Invariant: RECONCILED items must have a confidence score > 0
            if status == "RECONCILED" && score <= 0.0 {
                return Err(anyhow!("Invariant violation: RECONCILED taxon at index {} has 0 confidence", i));
            }

            // Invariant: UNMATCHED items must have a confidence score of 0
            if status == "UNMATCHED" && score > 0.0 {
                return Err(anyhow!("Invariant violation: UNMATCHED taxon at index {} has non-zero confidence", i));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod invariant_tests {
    use super::*;
    use arrow::datatypes::{Schema, Field, DataType};

    #[test]
    fn test_valid_reconciliation() {
        let schema = Arc::new(Schema::new(vec![
            Field::new("reconciliation_status", DataType::Utf8, false),
            Field::new("confidence_score", DataType::Float64, false),
        ]));
        let batch = RecordBatch::try_new(schema, vec![
            Arc::new(StringArray::from(vec!["RECONCILED", "UNMATCHED"])),
            Arc::new(Float64Array::from(vec![0.9, 0.0])),
        ]).unwrap();

        assert!(InvariantEngine::validate_reconciliation_invariants(&batch).is_ok());
    }

    #[test]
    fn test_invalid_reconciliation() {
        let schema = Arc::new(Schema::new(vec![
            Field::new("reconciliation_status", DataType::Utf8, false),
            Field::new("confidence_score", DataType::Float64, false),
        ]));
        let batch = RecordBatch::try_new(schema, vec![
            Arc::new(StringArray::from(vec!["RECONCILED"])),
            Arc::new(Float64Array::from(vec![0.0])),
        ]).unwrap();

        assert!(InvariantEngine::validate_reconciliation_invariants(&batch).is_err());
    }
}
