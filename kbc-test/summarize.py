import sys
import argparse
import json
import os
import glob
from collections import defaultdict

# Summarize results of different rulesets across all terms
def main():
    parser = argparse.ArgumentParser(description="Compare results of different rulesets by term.")
    parser.add_argument("dir", type=str, help="Directory containing JSON result files.")

    args = parser.parse_args()
    main_dir = args.dir
    base_dir = os.path.basename(os.path.normpath(main_dir))

    for entry in os.listdir(main_dir):
        dir_path = os.path.join(main_dir, entry)
        if os.path.isdir(dir_path):
            print(f"Processing subdirectory: {dir_path}")

        # Get all .jsonl files in directory
        files = glob.glob(os.path.join(dir_path, "*.jsonl"))

        if not files:
            print(f"No .jsonl files found in {dir_path}")
            sys.exit(1)

        fps = [(os.path.splitext(os.path.basename(fname))[0], open(fname, "r")) for fname in files]

        outfile = os.path.join(
            "results/eval/",
            f"summary_{base_dir}_{os.path.basename(dir_path)}.json"
        )

        os.makedirs(os.path.dirname(outfile), exist_ok=True)

        # Accumulators
        totals = defaultdict(lambda: defaultdict(float))
        counts = defaultdict(int)

        try:
            # zip() lets us read line by line from all files
            for lines in zip(*(fp for _, fp in fps)):
                objs = [json.loads(line) for line in lines]

                # Take base input weights/depth from the first file
                input_weight_simple = objs[0]["input_weight_simple"]
                input_weight_complex = objs[0]["input_weight_complex"]
                input_depth = objs[0]["input_depth"]

                # Update per ruleset
                for (name, _), obj in zip(fps, objs):
                    out_simple = obj["output_weight_simple"]
                    out_complex = obj["output_weight_complex"]
                    out_depth = obj["output_depth"]
                    time = obj.get("simplification_time", 0.0)

                    totals[name]["time"] += time
                    totals[name]["weight_simple_diff"] += input_weight_simple - out_simple
                    totals[name]["weight_complex_diff"] += input_weight_complex - out_complex
                    totals[name]["depth_diff"] += input_depth - out_depth
                    counts[name] += 1

        finally:
            for _, fp in fps:
                fp.close()

        # Compute averages
        averages = {}
        for name, metrics in totals.items():
            n = counts[name]
            averages[name] = {k: v / n for k, v in metrics.items()}

        # Final summary
        summary = {
            "totals": totals,
            "averages": averages,
            "counts": counts
        }

        # Write to JSON
        with open(outfile, "w") as out_f:
            json.dump(summary, out_f, indent=2)

if __name__ == '__main__':
    main()
