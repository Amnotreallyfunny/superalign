import os
import random
from typing import Optional


class BioDataGenerator:
    """Generates adversarial biological datasets for platform stress testing."""

    @staticmethod
    def generate_noisy_fasta(
        path: str, num_seqs: int = 100, length: int = 1000, noise_ratio: float = 0.1
    ) -> None:
        bases = ["A", "C", "G", "T"]
        with open(path, "w") as f:
            for i in range(num_seqs):
                # Simulate varied header noise
                headers = [
                    f">Taxon_{i}_Ref_v1.0",
                    f">taxon-{i}-sequence",
                    f">S.{i}_isolate_2026",
                    f">UNIDENTIFIED_ORGANISM_{i}",
                ]
                f.write(f"{random.choice(headers)}\n")

                seq = [random.choice(bases) for _ in range(length)]
                # Inject "Poison" noise (Gaps and Ambiguous codes)
                for _ in range(int(length * noise_ratio)):
                    idx = random.randint(0, length - 1)
                    seq[idx] = random.choice(["-", "N", "X", "?"])

                f.write("".join(seq) + "\n")

    @staticmethod
    def generate_locus_disjoint_set(
        dir_path: str,
        num_loci: int = 5,
        taxa_pool: Optional[list[str]] = None,
    ) -> None:
        if taxa_pool is None:
            taxa_pool = ["Human", "Mouse", "Roach", "Fly"]

        os.makedirs(dir_path, exist_ok=True)
        for i in range(num_loci):
            locus_taxa = random.sample(taxa_pool, random.randint(2, len(taxa_pool)))
            BioDataGenerator.generate_noisy_fasta(
                os.path.join(dir_path, f"locus_{i}.fasta"),
                num_seqs=len(locus_taxa),
                length=random.randint(500, 2000),
            )
