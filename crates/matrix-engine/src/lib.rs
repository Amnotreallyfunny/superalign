use anyhow::{Result};
use zarrs::{
    array::{Array, ArrayBuilder, DataType, FillValue},
    array_subset::ArraySubset,
    filesystem::FilesystemStore,
};
use std::sync::Arc;

pub struct MatrixEngine {
    store_path: String,
}

impl MatrixEngine {
    pub fn new(store_path: &str) -> Self {
        Self {
            store_path: store_path.to_string(),
        }
    }

    pub fn create_matrix(&self, num_taxa: u64, total_length: u64) -> Result<()> {
        let store = Arc::new(FilesystemStore::new(&self.store_path)?);
        
        let array = ArrayBuilder::new(
            vec![num_taxa, total_length],
            DataType::UInt8,
            vec![100, 1000].try_into()?,
            FillValue::from(b'-'),
        )
        .build(store, "/")?;
        
        array.store_metadata()?;
        Ok(())
    }

    pub fn write_chunk(
        &self,
        taxon_index: u64,
        start_pos: u64,
        data: &[u8],
    ) -> Result<()> {
        let store = Arc::new(FilesystemStore::new(&self.store_path)?);
        let array = Array::open(store, "/")?;
        
        let subset = ArraySubset::new_with_start_shape(
            vec![taxon_index, start_pos],
            vec![1, data.len() as u64],
        )?;
        
        array.store_array_subset(
            &subset,
            data,
        )?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_matrix_creation_and_writing() -> Result<()> {
        let dir = tempdir()?;
        let path = dir.path().to_str().unwrap();
        let engine = MatrixEngine::new(path);
        
        engine.create_matrix(10, 100)?;
        
        let data = b"ACGTACGT";
        engine.write_chunk(0, 0, data)?;
        
        Ok(())
    }
}
