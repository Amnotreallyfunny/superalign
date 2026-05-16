# SuperAlign Schema Contracts

## Overview
This package defines the canonical Apache Arrow schemas for the SuperAlign platform. These schemas act as the formal contract between Rust, Python, DuckDB, and Parquet layers.

## Canonical Schemas

### 1. TaxonEntity
Represents a biological entity (taxon) extracted from source data and reconciled against an ontology.

| Field | Type | Description |
| :--- | :--- | :--- |
| `entity_uuid` | Utf8 | Unique identifier for the entity |
| `raw_label` | Utf8 | Original label from source (e.g., FASTA header) |
| `normalized_label` | Utf8 | Label after normalization steps |
| `canonical_name` | Utf8 | Official name in the reference ontology |
| `ontology_id` | Utf8 | ID in the reference ontology (e.g., NCBI:txid9606) |
| `taxonomy_rank` | Utf8 | Rank (species, genus, etc.) |
| `lineage_array` | List<Utf8> | Full taxonomic path |
| `lineage_hash` | Utf8 | Deterministic hash of the lineage |
| `source_file` | Utf8 | Originating filename |
| `source_record_index`| Int64 | Index within the source file |
| `reconciliation_status`| Utf8 | Status (matched, fuzzy, unmatched) |
| `confidence_score` | Float64 | 0.0 to 1.0 confidence in the match |
| `pipeline_version` | Utf8 | Version of SuperAlign that created this record |
| `schema_version` | Utf8 | Version of this schema contract |
| `timestamp` | Timestamp | Creation time (UTC) |

### 2. MatchProvenance
Explains the reasoning and data behind a specific reconciliation event.

| Field | Type | Description |
| :--- | :--- | :--- |
| `provenance_uuid` | Utf8 | Unique ID for this audit record |
| `entity_uuid` | Utf8 | FK to TaxonEntity |
| `matching_algorithm`| Utf8 | Algorithm used (e.g., Jaro-Winkler) |
| `threshold_used` | Float64 | Minimum score required |
| `computed_distance` | Float64 | Final score of the match |
| `competing_candidates`| Utf8 | JSON list of alternative candidates |
| `selected_candidate` | Utf8 | The candidate chosen |
| `ontology_source` | Utf8 | Reference ontology version/source |
| `normalization_steps`| List<Utf8>| Transformations applied (e.g., lowercase) |
| `reconciliation_timestamp`| Timestamp | Time of execution |
| `execution_hash` | Utf8 | Hash of the execution context |

## Schema Evolution
- **Forward Compatibility:** New fields must be added as Nullable.
- **Breaking Changes:** Require an increment of the `schema_version` and a migration plan.
- **Serialization:** All records are serialized to Parquet for long-term storage and DuckDB ingestion.
