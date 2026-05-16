#[cfg(test)]
mod determinism_tests {
    use crate::MatrixEngine;
    use anyhow::Result;
    use tempfile::tempdir;
    use sha2::{Digest, Sha256};
    use std::fs;
    use std::path::Path;

    fn hash_directory(path: &Path) -> String {
        let mut hasher = Sha256::new();
        let mut entries: Vec<_> = fs::read_dir(path).unwrap().map(|r| r.unwrap()).collect();
        entries.sort_by_key(|e| e.path());
        for entry in entries {
            if entry.path().is_file() {
                hasher.update(fs::read(entry.path()).unwrap());
            }
        }
        hex::encode(hasher.finalize())
    }

    #[test]
    fn test_distributed_write_determinism() -> Result<()> {
        let taxa = vec!["t1".into(), "t2".into()];
        let loci = vec![("g1".into(), 10), ("g2".into(), 10)];
        
        let dir1 = tempdir()?;
        let engine1 = MatrixEngine::new(dir1.path().to_str().unwrap());
        let plan1 = engine1.plan_matrix(taxa.clone(), loci.clone());
        engine1.initialize_from_plan(&plan1)?;
        
        // Write order: t1 then t2
        engine1.write_taxon_locus(&plan1, "t1", "g1", b"AAAAAAAAAA")?;
        engine1.write_taxon_locus(&plan1, "t2", "g2", b"GGGGGGGGGG")?;
        let hash1 = hash_directory(dir1.path());

        let dir2 = tempdir()?;
        let engine2 = MatrixEngine::new(dir2.path().to_str().unwrap());
        let plan2 = engine2.plan_matrix(taxa, loci);
        engine2.initialize_from_plan(&plan2)?;
        
        // Write order: t2 then t1 (reversed)
        engine2.write_taxon_locus(&plan2, "t2", "g2", b"GGGGGGGGGG")?;
        engine2.write_taxon_locus(&plan2, "t1", "g1", b"AAAAAAAAAA")?;
        let hash2 = hash_directory(dir2.path());

        // Invariant: Final Zarr state must be identical regardless of write order
        assert_eq!(hash1, hash2, "Matrix state diverged due to write ordering");
        
        Ok(())
    }
}
