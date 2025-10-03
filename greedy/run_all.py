import subprocess
from pathlib import Path

# This script runs greedy on all combinations of rules and term files,
# and appends the results to results.csv
# This script was written using ChatGPT

# Paths
binary = Path("target/release/greedy")
rules_root = Path("../kbc/extended_rules/as_egg/")
terms_root = Path("../term_gen/sets/")
results_file = Path("results.csv")

with results_file.open("a") as out:
    # Iterate over all files in rules_root (recursively)
    for rule_file in rules_root.rglob("*"):
        if not rule_file.is_file():
            continue

        # Iterate over all files in terms_root
        for term_file in terms_root.iterdir():
            if not term_file.is_file():
                continue

            # Run greedy with (rule_file, term_file)
            result = subprocess.run(
                [str(binary), str(rule_file), str(term_file)],
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
            )

            # Extract filename stems (without extension)
            rule_stem = rule_file.stem
            term_stem = term_file.stem

            # Take stdout, strip newlines/spaces
            output = result.stdout.strip().replace("\n", " ")

            # Append one line
            out.write(f"{rule_stem},{term_stem},{output}\n")

print(f"Done. Results appended to {results_file}")