
import click
import pyarrow as pa
from rich.console import Console
from rich.progress import Progress, SpinnerColumn, TextColumn
from rich.table import Table

import superalign

console = Console()

@click.group()
def main() -> None:
    """SuperAlign CLI - Production-grade phylogenomics infrastructure."""
    pass

@main.command()
@click.argument('fasta_path', type=click.Path(exists=True))
@click.option('--batch-size', default=1024, help='Number of sequences per batch')
def ingest(fasta_path: str, batch_size: int) -> None:
    """Ingest and hash genomic sequences from a FASTA file."""
    console.print(
        f"[bold blue]SUPERALIGN[/bold blue] | Starting Ingestion of {fasta_path}..."
    )
    
    table = Table(title="Sequence Metadata Extract")
    table.add_column("UUID", style="cyan")
    table.add_column("Label", style="white")
    table.add_column("Length (bp)", justify="right", style="green")
    table.add_column("SHA-256", style="dim")

    with Progress(
        SpinnerColumn(),
        TextColumn("[progress.description]{task.description}"),
        transient=True,
    ) as progress:
        progress.add_task(description="Streaming and Hashing...", total=None)
        for entities, metadata in superalign.parse_fasta(fasta_path, batch_size):
            # Select supported columns for join (excluding list types for now)
            ent_cols = ["entity_uuid", "raw_label"]
            joined = entities.select(ent_cols).join(metadata, keys="entity_uuid")
            for row in joined.to_pylist():
                table.add_row(
                    row["entity_uuid"][:8],
                    row["raw_label"][:30],
                    str(row["sequence_len"]),
                    row["sequence_hash"][:16] + "..."
                )
    
    console.print(table)
    console.print(
        f"[bold green]SUCCESS[/bold green] | Processed {len(joined)} sequences locally."
    )

@main.command()
@click.argument('manifest_path', type=click.Path(exists=True))
@click.option('--threshold', default=0.85, help='Fuzzy match threshold')
def reconcile(manifest_path: str, threshold: float) -> None:
    """Reconcile sequence headers against taxonomic ontologies."""
    console.print(
        f"[bold blue]SUPERALIGN[/bold blue] | "
        f"Reconciling against NCBI Ontology (Threshold: {threshold})..."
    )
    
    # In a real CLI we would load the parquet manifest
    # For this demo, we'll simulate the workflow
    table = Table(title="Taxonomic Reconciliation Results")
    table.add_column("Raw Label", style="white")
    table.add_column("Canonical Match", style="italic")
    table.add_column("Ontology ID", style="cyan")
    table.add_column("Score", justify="right")

    # Mocking the process using the core engine logic
    dummy_data = pa.Table.from_pylist([
        {"raw_label": "Homo sapiens", "entity_uuid": "uuid1"},
        {"raw_label": "Mus musculus", "entity_uuid": "uuid2"},
        {"raw_label": "SARS-CoV-2", "entity_uuid": "uuid3"}
    ])
    
    reconciled, _ = superalign.reconcile(dummy_data, threshold=threshold)
    
    for row in reconciled.to_pylist():
        score_color = "green" if row["confidence_score"] > 0.9 else "yellow"
        table.add_row(
            "N/A", # Simulating join
            row["canonical_name"] or "UNMATCHED",
            row["ontology_id"] or "N/A",
            f"[{score_color}]{row['confidence_score']:.4f}[/{score_color}]"
        )
    
    console.print(table)

if __name__ == "__main__":
    main()
