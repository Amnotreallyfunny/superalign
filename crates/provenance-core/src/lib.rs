use anyhow::{Result};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use uuid::Uuid;
use std::sync::Arc;
use arrow::array::{
    ArrayRef, ListBuilder, StringBuilder, TimestampMicrosecondBuilder,
};
use arrow::record_batch::RecordBatch;
use parquet::arrow::arrow_writer::ArrowWriter;
use std::fs::File;

use superalign_schemas::{PROCESS_PROVENANCE_SCHEMA, get_current_schema_version};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProcessNode {
    pub process_uuid: String,
    pub parent_process_uuid: Option<String>,
    pub operation_name: String,
    pub input_hashes: Vec<String>,
    pub output_hashes: Vec<String>,
    pub execution_context_hash: String,
    pub plugin_id: Option<String>,
    pub pipeline_version: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProvenanceManifest {
    pub manifest_id: String,
    pub nodes: Vec<ProcessNode>,
    pub metadata: HashMap<String, String>,
}

pub struct ProvenanceManager {
    nodes: Vec<ProcessNode>,
    current_pipeline_version: String,
}

impl ProvenanceManager {
    pub fn new(pipeline_version: &str) -> Self {
        Self {
            nodes: Vec::new(),
            current_pipeline_version: pipeline_version.to_string(),
        }
    }

    pub fn record_process(
        &mut self,
        operation: &str,
        inputs: Vec<String>,
        outputs: Vec<String>,
        parent_uuid: Option<String>,
        plugin: Option<String>,
    ) -> String {
        let uuid = Uuid::new_v4().to_string();
        
        // Compute execution context hash (simplified for now)
        let mut hasher = Sha256::new();
        hasher.update(operation.as_bytes());
        for h in &inputs { hasher.update(h.as_bytes()); }
        let execution_hash = hex::encode(hasher.finalize());

        let node = ProcessNode {
            process_uuid: uuid.clone(),
            parent_process_uuid: parent_uuid,
            operation_name: operation.to_string(),
            input_hashes: inputs,
            output_hashes: outputs,
            execution_context_hash: execution_hash,
            plugin_id: plugin,
            pipeline_version: self.current_pipeline_version.clone(),
            timestamp: Utc::now(),
        };

        self.nodes.push(node);
        uuid
    }

    pub fn to_json_report(&self) -> Result<String> {
        let manifest = ProvenanceManifest {
            manifest_id: Uuid::new_v4().to_string(),
            nodes: self.nodes.clone(),
            metadata: [("schema_version".to_string(), get_current_schema_version().to_string())]
                .into_iter().collect(),
        };
        Ok(serde_json::to_string_pretty(&manifest)?)
    }

    pub fn export_parquet(&self, path: &str) -> Result<()> {
        let mut uuids = StringBuilder::new();
        let mut parents = StringBuilder::new();
        let mut operations = StringBuilder::new();
        let mut input_hashes_list = ListBuilder::new(StringBuilder::new());
        let mut output_hashes_list = ListBuilder::new(StringBuilder::new());
        let mut exec_hashes = StringBuilder::new();
        let mut plugins = StringBuilder::new();
        let mut versions = StringBuilder::new();
        let mut timestamps = TimestampMicrosecondBuilder::new().with_timezone("UTC");

        for node in &self.nodes {
            uuids.append_value(&node.process_uuid);
            match &node.parent_process_uuid {
                Some(p) => parents.append_value(p),
                None => parents.append_null(),
            }
            operations.append_value(&node.operation_name);
            
            let ih_builder = input_hashes_list.values();
            for h in &node.input_hashes { ih_builder.append_value(h); }
            input_hashes_list.append(true);

            let oh_builder = output_hashes_list.values();
            for h in &node.output_hashes { oh_builder.append_value(h); }
            output_hashes_list.append(true);

            exec_hashes.append_value(&node.execution_context_hash);
            match &node.plugin_id {
                Some(p) => plugins.append_value(p),
                None => plugins.append_null(),
            }
            versions.append_value(&node.pipeline_version);
            timestamps.append_value(node.timestamp.timestamp_micros());
        }

        let batch = RecordBatch::try_new(
            PROCESS_PROVENANCE_SCHEMA.clone(),
            vec![
                Arc::new(uuids.finish()) as ArrayRef,
                Arc::new(parents.finish()) as ArrayRef,
                Arc::new(operations.finish()) as ArrayRef,
                Arc::new(input_hashes_list.finish()) as ArrayRef,
                Arc::new(output_hashes_list.finish()) as ArrayRef,
                Arc::new(exec_hashes.finish()) as ArrayRef,
                Arc::new(plugins.finish()) as ArrayRef,
                Arc::new(versions.finish()) as ArrayRef,
                Arc::new(timestamps.finish()) as ArrayRef,
            ]
        )?;

        let file = File::create(path)?;
        let mut writer = ArrowWriter::try_new(file, PROCESS_PROVENANCE_SCHEMA.clone(), None)?;
        writer.write(&batch)?;
        writer.close()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provenance_recording() {
        let mut pm = ProvenanceManager::new("0.1.0");
        let id1 = pm.record_process("FASTA_PARSE", vec!["file1_hash".into()], vec!["out1_hash".into()], None, None);
        pm.record_process("RECONCILE", vec!["out1_hash".into()], vec!["reconciled_hash".into()], Some(id1), Some("tax_v1".into()));
        
        assert_eq!(pm.nodes.len(), 2);
        let report = pm.to_json_report().unwrap();
        assert!(report.contains("FASTA_PARSE"));
        assert!(report.contains("RECONCILE"));
    }
}
