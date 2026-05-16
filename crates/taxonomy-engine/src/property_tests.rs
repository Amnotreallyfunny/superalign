#[cfg(test)]
mod property_tests {
    use crate::{TaxonomyDb, Reconciler};
    use proptest::prelude::*;
    use arrow::array::{StringArray, ArrayRef};
    use arrow::record_batch::RecordBatch;
    use std::sync::Arc;

    fn noisy_label_strategy() -> impl Strategy<Value = String> {
        prop_oneof![
            // Exact matches with capitalization variance
            prop::string::string_regex("[hH]omo [sS]apiens").unwrap(),
            // Common delimiters
            prop::string::string_regex("Homo[ _.]sapiens").unwrap(),
            // Incomplete labels
            prop::string::string_regex("H\\. sapiens").unwrap(),
            // Random noise around the label
            prop::string::string_regex(".*Homo sapiens.*").unwrap(),
        ]
    }

    proptest! {
        #[test]
        fn test_reconciliation_resilience(label in noisy_label_strategy()) {
            let db = TaxonomyDb::new_in_memory().unwrap();
            db.load_sample_ontology().unwrap();
            let reconciler = Reconciler::new(db, 0.6); // Lower threshold for fuzzing

            let batch = RecordBatch::try_new(
                Arc::new(arrow::datatypes::Schema::new(vec![
                    arrow::datatypes::Field::new("raw_label", arrow::datatypes::DataType::Utf8, false),
                    arrow::datatypes::Field::new("entity_uuid", arrow::datatypes::DataType::Utf8, false),
                ])),
                vec![
                    Arc::new(StringArray::from(vec![label.clone()])) as ArrayRef,
                    Arc::new(StringArray::from(vec!["uuid1"])) as ArrayRef,
                ]
            ).unwrap();

            // The test passes if the reconciler doesn't crash and returns a valid result
            let (reconciled, _) = reconciler.reconcile_batch(&batch).unwrap();
            assert_eq!(reconciled.num_rows(), 1);
        }
    }
}
