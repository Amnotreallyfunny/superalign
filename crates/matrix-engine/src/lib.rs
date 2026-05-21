use anyhow::{Context, Result};
use zarrs::{
    array::{Array, ArrayBuilder, DataType, FillValue},
    array_subset::ArraySubset,
    filesystem::FilesystemStore,
};
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use duckdb::{params, Connection, Config, AccessMode};
use lru::LruCache;
use std::num::NonZeroUsize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocusPartition {
    pub locus_name: String,
    pub start_offset: u64,
    pub length: u64,
}

/// A Bounded-Memory Coordinate Manifest.
/// Replaces large in-memory HashMaps with a persistent lookup + hot cache.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritePlan {
    pub total_taxa: u64,
    pub total_length: u64,
    pub partitions: Vec<LocusPartition>,
    pub index_db_path: String, // Path to the DuckDB persistent index
}

pub struct PersistentIndex {
    conn: Connection,
}

impl PersistentIndex {
    pub fn new(path: &str, read_only: bool) -> Result<Self> {
        let config = Config::default()
            .access_mode(if read_only { AccessMode::ReadOnly } else { AccessMode::ReadWrite })?;
        
        let conn = Connection::open_with_flags(path, config)?;
        
        if !read_only {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS taxon_index (
                    taxon_id TEXT PRIMARY KEY,
                    row_idx BIGINT NOT NULL
                )",
                [],
            )?;
            conn.execute("CREATE INDEX IF NOT EXISTS idx_taxon_id ON taxon_index(taxon_id)", [])?;
        }
        Ok(Self { conn })
    }

    pub fn insert_taxon(&self, taxon_id: &str, row_idx: u64) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO taxon_index VALUES (?, ?)",
            params![taxon_id, row_idx],
        )?;
        Ok(())
    }

    pub fn get_row_idx(&self, taxon_id: &str) -> Result<Option<u64>> {
        let mut stmt = self.conn.prepare("SELECT row_idx FROM taxon_index WHERE taxon_id = ?")?;
        let mut rows = stmt.query(params![taxon_id])?;
        if let Some(row) = rows.next()? {
            let idx: i64 = row.get(0)?;
            Ok(Some(idx as u64))
        } else {
            Ok(None)
        }
    }
}

pub struct MatrixEngine {
    store_path: String,
    index_cache: LruCache<String, u64>, // Hot cache for frequent lookups
}

impl MatrixEngine {
    pub fn new(store_path: &str) -> Self {
        Self {
            store_path: store_path.to_string(),
            index_cache: LruCache::new(NonZeroUsize::new(10000).unwrap()), // Bounded to 10k hot entries
        }
    }

    /// Generate a write plan using a persistent index.
    /// This is O(working_set) memory, not O(T).
    pub fn plan_matrix(
        &self,
        index_db_path: &str,
        taxa_list: Vec<String>,
        loci: Vec<(String, u64)>,
    ) -> Result<WritePlan> {
        let index = PersistentIndex::new(index_db_path, false)?;
        
        for (i, taxon) in taxa_list.into_iter().enumerate() {
            index.insert_taxon(&taxon, i as u64)?;
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

        Ok(WritePlan {
            total_taxa: index.conn.query_row("SELECT count(*) FROM taxon_index", [], |r| r.get::<_, i64>(0))? as u64,
            total_length: current_offset,
            partitions,
            index_db_path: index_db_path.to_string(),
        })
    }

    pub fn initialize_from_plan(&self, plan: &WritePlan) -> Result<()> {
        let store = Arc::new(FilesystemStore::new(&self.store_path)?);
        
        // Parallel-Safe Chunking Strategy:
        // Set row chunk size to 1. This ensures each taxon has its own set of chunks,
        // allowing multiple workers to write different taxa simultaneously without locking.
        let chunk_shape = vec![1, 10000.min(plan.total_length)];
        
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

    pub fn write_taxon_locus(
        &mut self,
        plan: &WritePlan,
        taxon_id: &str,
        locus_name: &str,
        data: &[u8],
    ) -> Result<()> {
        // 1. Check Hot Cache
        let row_index = if let Some(idx) = self.index_cache.get(taxon_id) {
            *idx
        } else {
            // 2. Persistent Lookup (O(1) seek) - READ ONLY for worker concurrency
            let index = PersistentIndex::new(&plan.index_db_path, true)?;
            let idx = index.get_row_idx(taxon_id)?
                .with_context(|| format!("Taxon {} not in persistent index", taxon_id))?;
            self.index_cache.put(taxon_id.to_string(), idx);
            idx
        };
        
        let partition = plan.partitions.iter().find(|p| p.locus_name == locus_name)
            .with_context(|| format!("Locus {} not in plan", locus_name))?;

        if data.len() as u64 != partition.length {
            return Err(anyhow::anyhow!("Data length mismatch for {}", locus_name));
        }

        let store = Arc::new(FilesystemStore::new(&self.store_path)?);
        let array = Array::open(store, "/")?;
        
        let subset = ArraySubset::new_with_start_shape(
            vec![row_index, partition.start_offset],
            vec![1, partition.length],
        )?;
        
        array.store_array_subset(&subset, data)?;
        Ok(())
    }
}

mod determinism_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_persistent_indexing() -> Result<()> {
        let dir = tempdir()?;
        let store_path = dir.path().join("matrix.zarr");
        let index_path = dir.path().join("index.db");
        
        let mut engine = MatrixEngine::new(store_path.to_str().unwrap());
        
        let taxa = vec!["t1".to_string(), "t2".to_string()];
        let loci = vec![("g1".to_string(), 5)];
        
        let plan = engine.plan_matrix(index_path.to_str().unwrap(), taxa, loci)?;
        engine.initialize_from_plan(&plan)?;
        
        engine.write_taxon_locus(&plan, "t1", "g1", b"AAAAA")?;
        engine.write_taxon_locus(&plan, "t2", "g1", b"CCCCC")?;
        
        Ok(())
    }
}
