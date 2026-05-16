use anyhow::{Context, Result};
use zarrs::{
    array::{Array, ArrayBuilder, DataType, FillValue},
    array_subset::ArraySubset,
    filesystem::FilesystemStore,
};
use std::collections::HashMap;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocusPartition {
    pub locus_name: String,
    pub start_offset: u64,
    pub length: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WritePlan {
    pub total_taxa: u64,
    pub total_length: u64,
    pub partitions: Vec<LocusPartition>,
    pub taxon_id_to_index: HashMap<String, u64>,
}

pub struct MatrixEngine {
    store_path: String,
}

impl MatrixEngine {
    pub fn new(store_path: &str) -> Self {
        Self {
            store_path: store_path.to_string(),
        }
    }

    /// Perform a dry-run scan to determine matrix dimensions and generate a write plan.
    /// In a real scenario, this would iterate over all alignment files.
    pub fn plan_matrix(
        &self,
        taxa_list: Vec<String>,
        loci: Vec<(String, u64)>, // (name, length)
    ) -> WritePlan {
        let mut taxon_id_to_index = HashMap::new();
        for (i, taxon) in taxa_list.into_iter().enumerate() {
            taxon_id_to_index.insert(taxon, i as u64);
        }

        let mut current_offset = 0;
        let mut partitions = Vec::new();
        for (name, len) in loci {
            partitions.push(LocusPartition {
                locus_name: name,
                start_offset: current_offset,
                length: len,
            });
            current_offset += len;
        }

        WritePlan {
            total_taxa: taxon_id_to_index.len() as u64,
            total_length: current_offset,
            partitions,
            taxon_id_to_index,
        }
    }

    /// Pre-allocate a Zarr array based on a WritePlan.
    /// This is HPC-safe as it only writes metadata initially.
    pub fn initialize_from_plan(&self, plan: &WritePlan) -> Result<()> {
        let store = Arc::new(FilesystemStore::new(&self.store_path)?);
        
        // Optimize chunk size for HPC parallel filesystems (many taxa, chunked by loci)
        let chunk_shape = vec![plan.total_taxa, 10000.min(plan.total_length)];
        
        let array = ArrayBuilder::new(
            vec![plan.total_taxa, plan.total_length],
            DataType::UInt8,
            chunk_shape.try_into()?,
            FillValue::from(b'-'),
        )
        .build(store, "/")?;
        
        array.store_metadata()?;
        Ok(())
    }

    /// Distributed-safe write: write a specific taxon's sequence for a specific locus.
    /// Since workers write to unique (taxon, locus) coordinates, this is lock-free.
    pub fn write_taxon_locus(
        &self,
        plan: &WritePlan,
        taxon_id: &str,
        locus_name: &str,
        data: &[u8],
    ) -> Result<()> {
        let taxon_index = plan.taxon_id_to_index.get(taxon_id)
            .with_context(|| format!("Taxon {} not in plan", taxon_id))?;
        
        let partition = plan.partitions.iter().find(|p| p.locus_name == locus_name)
            .with_context(|| format!("Locus {} not in plan", locus_name))?;

        if data.len() as u64 != partition.length {
            return Err(anyhow::anyhow!(
                "Data length {} does not match planned length {} for locus {}",
                data.len(), partition.length, locus_name
            ));
        }

        let store = Arc::new(FilesystemStore::new(&self.store_path)?);
        let array = Array::open(store, "/")?;
        
        let subset = ArraySubset::new_with_start_shape(
            vec![*taxon_index, partition.start_offset],
            vec![1, partition.length],
        )?;
        
        array.store_array_subset(&subset, data)?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_distributed_planning_and_writing() -> Result<()> {
        let dir = tempdir()?;
        let path = dir.path().to_str().unwrap();
        let engine = MatrixEngine::new(path);
        
        let taxa = vec!["taxon1".to_string(), "taxon2".to_string()];
        let loci = vec![("geneA".to_string(), 10), ("geneB".to_string(), 20)];
        
        let plan = engine.plan_matrix(taxa, loci);
        assert_eq!(plan.total_length, 30);
        
        engine.initialize_from_plan(&plan)?;
        
        // Simulating distributed workers writing different parts
        engine.write_taxon_locus(&plan, "taxon1", "geneA", b"ATGCATGCAT")?;
        engine.write_taxon_locus(&plan, "taxon2", "geneB", b"GGGGGGGGGGGGGGGGGGGG")?;
        
        Ok(())
    }
}
