import os
import time
import psutil
import pyarrow as pa
import superalign
import uuid
import random
import hashlib
import shutil
import multiprocessing

def get_memory_usage():
    process = psutil.Process(os.getpid())
    return process.memory_info().rss / 1024 / 1024  # MB

def compute_directory_hash(path):
    """Computes a recursive SHA-256 hash of a directory structure."""
    hasher = hashlib.sha256()
    for root, dirs, files in os.walk(path):
        for f in sorted(files):
            f_path = os.path.join(root, f)
            with open(f_path, "rb") as fh:
                hasher.update(fh.read())
    return hasher.hexdigest()

def worker_task(store_path, index_db_path, plan_json, taxon_id, locus_name, sequence):
    """Simulates a remote HPC worker writing a single taxon-locus block."""
    engine = superalign.MatrixEngine(store_path)
    plan = superalign.WritePlan.from_json(plan_json)
    engine.write_taxon_locus(plan, taxon_id, locus_name, sequence.encode())

def run_brutal_reconciliation_test():
    print("--- [NATIONAL LAB AUDIT: BIOLOGICAL HIERARCHY] ---")
    
    headers = [
        ("h1", "Homo sapiens [taxid:9606]"),  # Priority 1: TaxID
        ("h2", "NC_000067.1"),                # Priority 2: Accession (Mus musculus)
        ("h3", "human"),                      # Priority 3: Synonym
        ("h4", "Mus muscul"),                 # Priority 4: Fuzzy Match
    ]
    
    data = [{"entity_uuid": str(uuid.uuid4()), "raw_label": h} for _, h in headers]
    table = pa.Table.from_pylist(data)
    
    reconciled, prov = superalign.reconcile(table)
    algos = prov.column("matching_algorithm").to_pylist()
    expected_algos = ["NCBI_TAXID_EXTRACT", "ACCESSION_GROUNDING", "EXACT_ONTOLOGY_MATCH", "JARO_WINKLER_FALLBACK"]
    
    for i, (expected, actual) in enumerate(zip(expected_algos, algos)):
        print(f"Header: {headers[i][1]:<30} | Match: {actual}")
        assert expected == actual, f"Hierarchy violation at index {i}"

    print("✅ Verified: Biological Identity Hierarchy Enforced.\n")

def run_scale_memory_test(count=100000):
    print(f"--- [NATIONAL LAB AUDIT: BOUNDED MEMORY SCALE ({count} taxa)] ---")
    
    mid_mem = get_memory_usage()
    data = [{"entity_uuid": str(uuid.uuid4()), "raw_label": f"Exp_Taxon_{i}_{uuid.uuid4().hex[:8]}"} for i in range(count)]
    table = pa.Table.from_pylist(data)
    
    print("Executing massive reconciliation...")
    start_time = time.time()
    batch_size = 10000
    for i in range(0, count, batch_size):
        superalign.reconcile(table.slice(i, batch_size))
        
    final_mem = get_memory_usage()
    print(f"Memory Delta: {final_mem - mid_mem:.2f} MB | Time: {time.time() - start_time:.2f}s")
    
    # Assert RSS doesn't balloon (RefSeq scale safety)
    assert (final_mem - mid_mem) < 300, "Memory Boundary Violation detected."
    print("✅ Verified: Working set memory bounded.\n")

def run_distributed_determinism_test():
    print("--- [NATIONAL LAB AUDIT: DISTRIBUTED DETERMINISM] ---")
    
    root_dir = "tests/national_lab_temp"
    if os.path.exists(root_dir): shutil.rmtree(root_dir)
    os.makedirs(root_dir)
    
    store_path = os.path.join(root_dir, "matrix.zarr")
    index_db_path = os.path.join(root_dir, "index.db")
    
    taxa = [f"taxon_{i}" for i in range(50)]
    loci = [("gene_1", 100), ("gene_2", 100)]
    
    engine = superalign.MatrixEngine(store_path)
    plan = engine.plan_matrix(index_db_path, taxa, loci)
    engine.initialize_from_plan(plan)
    plan_json = plan.to_json()
    
    tasks = []
    for t in taxa:
        for loc, length in loci:
            seq = "A" * length
            tasks.append((store_path, index_db_path, plan_json, t, loc, seq))
            
    random.shuffle(tasks)
    
    print(f"Launching {len(tasks)} concurrent tasks...")
    with multiprocessing.Pool(processes=8) as pool:
        pool.starmap(worker_task, tasks)
        
    distributed_hash = compute_directory_hash(store_path)
    print(f"Matrix Checksum: {distributed_hash}")
    
    # Run reference
    ref_dir = "tests/national_lab_ref"
    if os.path.exists(ref_dir): shutil.rmtree(ref_dir)
    os.makedirs(ref_dir)
    ref_store = os.path.join(ref_dir, "ref.zarr")
    ref_index = os.path.join(ref_dir, "ref.db")
    
    engine_ref = superalign.MatrixEngine(ref_store)
    plan_ref = engine_ref.plan_matrix(ref_index, taxa, loci)
    engine_ref.initialize_from_plan(plan_ref)
    for t, l, s in [(t, loc, "A"*length) for t in taxa for loc, length in loci]:
        engine_ref.write_taxon_locus(plan_ref, t, l, s.encode())
        
    ref_hash = compute_directory_hash(ref_store)
    assert distributed_hash == ref_hash, "State Divergence: Parallel write corrupted the matrix."
    
    print("✅ Verified: Bit-for-bit parity across parallel workers.\n")
    shutil.rmtree(root_dir)
    shutil.rmtree(ref_dir)

if __name__ == "__main__":
    print("=== SUPERALIGN SCIENTIFIC VALIDATION: NATIONAL LAB STANDARDS ===\n")
    try:
        run_brutal_reconciliation_test()
        run_scale_memory_test(count=100000)
        run_distributed_determinism_test()
        print("=== VERDICT: SUPERALIGN INFRASTRUCTURE FULLY VERIFIED ===")
    except Exception as e:
        print(f"\n❌ AUDIT FAILED: {str(e)}")
        import traceback
        traceback.print_exc()
        exit(1)
