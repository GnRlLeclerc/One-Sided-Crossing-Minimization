"""Generate the output graphs for the PACE 2024 challenge."""

import os
import subprocess

ALGORITHM = "iter-barycenter"
DATASETS = ["tiny", "medium", "large"]
EXECUTABLE = ["cargo", "run", "--release", "--bin", "ocm-cli", "--", "-a", ALGORITHM]

if __name__ == "__main__":
    for dataset in DATASETS:
        files = os.listdir(f"datasets/{dataset}")

        for file in files:
            print(f"\rGenerating output graph for {dataset}/{file}...", end="")
            relative_path = f"datasets/{dataset}/{file}"
            subprocess.run(
                EXECUTABLE
                + [relative_path, "--output-file", f"output/{dataset}/{file}"],
                check=True,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
            )
