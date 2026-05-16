use anyhow::{Context, Result};
use arrow::record_batch::RecordBatch;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use arrow::datatypes::Schema;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PluginSignature {
    pub id: String,
    pub name: String,
    pub version: String,
    pub input_schema_hash: String,
    pub output_schema_hash: String,
    pub capability: Vec<String>,
}

pub trait SuperAlignPlugin: Send + Sync {
    /// Return the immutable signature of the plugin
    fn signature(&self) -> PluginSignature;

    /// Execute the plugin logic on an Arrow RecordBatch
    /// Pure function: No access to I/O or storage
    fn run(&self, input: &RecordBatch) -> Result<RecordBatch>;

    /// Validate if the input schema matches the plugin's requirements
    fn validate_schema(&self, schema: &Arc<Schema>) -> Result<()>;
}

pub struct PluginRuntime {
    plugins: Vec<Box<dyn SuperAlignPlugin>>,
}

impl PluginRuntime {
    pub fn new() -> Self {
        Self { plugins: Vec::new() }
    }

    pub fn register_plugin(&mut self, plugin: Box<dyn SuperAlignPlugin>) {
        self.plugins.push(plugin);
    }

    pub fn execute_plugin(
        &self,
        plugin_id: &str,
        input: &RecordBatch,
    ) -> Result<RecordBatch> {
        let plugin = self.plugins.iter()
            .find(|p| p.signature().id == plugin_id)
            .with_context(|| format!("Plugin {} not found", plugin_id))?;

        plugin.validate_schema(&input.schema())?;
        
        let output = plugin.run(input)?;
        
        Ok(output)
    }

    pub fn get_signatures(&self) -> Vec<PluginSignature> {
        self.plugins.iter().map(|p| p.signature()).collect()
    }
}

// Example Plugin Implementation
pub struct HeaderNormalizerPlugin {
    signature: PluginSignature,
}

impl HeaderNormalizerPlugin {
    pub fn new() -> Self {
        Self {
            signature: PluginSignature {
                id: "builtin.header_normalizer".to_string(),
                name: "Header Normalizer".to_string(),
                version: "0.1.0".to_string(),
                input_schema_hash: "taxon_entity_v1".to_string(),
                output_schema_hash: "taxon_entity_v1".to_string(),
                capability: vec!["normalization".to_string()],
            },
        }
    }
}

impl SuperAlignPlugin for HeaderNormalizerPlugin {
    fn signature(&self) -> PluginSignature {
        self.signature.clone()
    }

    fn run(&self, input: &RecordBatch) -> Result<RecordBatch> {
        // In a real implementation, we would use Arrow compute kernels
        // to normalize strings (e.g., lowercase).
        // For the MVP, we just return the input batch to demonstrate the interface.
        Ok(input.clone())
    }

    fn validate_schema(&self, _schema: &Arc<Schema>) -> Result<()> {
        // Basic check for required columns
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow::array::StringArray;
    use arrow::datatypes::{DataType, Field, Schema};

    #[test]
    fn test_plugin_runtime() -> Result<()> {
        let mut runtime = PluginRuntime::new();
        runtime.register_plugin(Box::new(HeaderNormalizerPlugin::new()));

        let schema = Arc::new(Schema::new(vec![
            Field::new("raw_label", DataType::Utf8, false),
        ]));
        let batch = RecordBatch::try_new(
            schema,
            vec![Arc::new(StringArray::from(vec!["Homo sapiens"]))],
        )?;

        let result = runtime.execute_plugin("builtin.header_normalizer", &batch)?;
        assert_eq!(result.num_rows(), 1);
        
        Ok(())
    }
}
