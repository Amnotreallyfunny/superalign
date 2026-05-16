use arrow::datatypes::{Schema, Field};
use anyhow::{Result, anyhow};
use std::sync::Arc;

/// Validates that a given schema matches a reference schema exactly.
/// Checks field names, types, and nullability.
pub fn validate_schema_contract(actual: &Schema, expected: &Schema) -> Result<()> {
    if actual.fields().len() != expected.fields().len() {
        return Err(anyhow!("Schema field count mismatch: expected {}, got {}", 
            expected.fields().len(), actual.fields().len()));
    }

    for (i, (actual_field, expected_field)) in actual.fields().iter().zip(expected.fields().iter()).enumerate() {
        if actual_field.name() != expected_field.name() {
            return Err(anyhow!("Field name mismatch at index {}: expected '{}', got '{}'", 
                i, expected_field.name(), actual_field.name()));
        }
        if actual_field.data_type() != expected_field.data_type() {
            return Err(anyhow!("Field type mismatch for '{}': expected {:?}, got {:?}", 
                expected_field.name(), expected_field.data_type(), actual_field.data_type()));
        }
        if actual_field.is_nullable() != expected_field.is_nullable() {
            return Err(anyhow!("Field nullability mismatch for '{}': expected {}, got {}", 
                expected_field.name(), expected_field.is_nullable(), actual_field.is_nullable()));
        }
    }

    Ok(())
}

#[cfg(test)]
mod contract_tests {
    use super::*;
    use crate::TAXON_ENTITY_SCHEMA;
    use arrow::datatypes::{DataType, Field};

    #[test]
    fn test_contract_validation_success() {
        validate_schema_contract(&TAXON_ENTITY_SCHEMA, &TAXON_ENTITY_SCHEMA).unwrap();
    }

    #[test]
    fn test_contract_validation_failure() {
        let bad_schema = Schema::new(vec![
            Field::new("wrong_name", DataType::Utf8, false),
        ]);
        assert!(validate_schema_contract(&bad_schema, &TAXON_ENTITY_SCHEMA).is_err());
    }
}
