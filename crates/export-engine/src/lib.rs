use anyhow::{Result};
use std::fs::File;
use std::io::Write;
use zarrs::{
    array::Array,
    filesystem::FilesystemStore,
};
use std::sync::Arc;

pub struct ExportEngine {
    store_path: String,
}

impl ExportEngine {
    pub fn new(store_path: &str) -> Self {
        Self {
            store_path: store_path.to_string(),
        }
    }

    /// Flatten a Zarr supermatrix into a legacy FASTA file.
    /// This uses streaming chunk reads to avoid OOM.
    pub fn to_fasta(&self, output_path: &str, taxon_names: Vec<String>) -> Result<()> {
        let store = Arc::new(FilesystemStore::new(&self.store_path)?);
        let array = Array::open(store, "/")?;
        let mut out_file = File::create(output_path)?;

        for (i, name) in taxon_names.into_iter().enumerate() {
            let shape = array.shape();
            let subset = zarrs::array_subset::ArraySubset::new_with_start_shape(
                vec![i as u64, 0],
                vec![1, shape[1]],
            )?;
            
            let data = array.retrieve_array_subset(&subset)?;
            
            // into_fixed() on ArrayBytes returns a Result<Cow<[u8]>, CodecError> in 0.17
            let bytes = data.into_fixed()
                .map_err(|e| anyhow::anyhow!("Zarr codec error: {}", e))?;
            
            writeln!(out_file, ">{}", name)?;
            let seq = String::from_utf8_lossy(&bytes);
            writeln!(out_file, "{}", seq)?;
        }

        Ok(())
    }

    pub fn to_phylip(&self, _output_path: &str) -> Result<()> {
        Ok(())
    }
}
