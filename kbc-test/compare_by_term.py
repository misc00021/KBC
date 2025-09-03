import sys
import argparse
import json
import os
import glob

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
            f"comp_by_term_{base_dir}_{os.path.basename(dir_path)}.json"
        )

        os.makedirs(os.path.dirname(outfile), exist_ok=True)

        all_results = []

        try:
            # zip() lets us read line by line from all files
            for lines in zip(*(fp for _, fp in fps)):
                objs = [json.loads(line) for line in lines]

                # Take base info from the first file
                base_term = objs[0]["original_term"]
                input_weight_simple = objs[0]["input_weight_simple"]
                input_weight_complex = objs[0]["input_weight_complex"]
                input_depth = objs[0]["input_depth"]

                combined = {
                    "base_term": base_term,
                    "input_weight_simple": input_weight_simple,
                    "input_weight_complex": input_weight_complex,
                    "input_depth": input_depth,
                }

                # Add each file's results under its own key
                for (name, _), obj in zip(fps, objs):
                    combined[name] = {
                        "output_term": obj["simplified_term"],
                        "output_weight_simple": obj["output_weight_simple"],
                        "output_weight_complex": obj["output_weight_complex"],
                        "output_depth": obj["output_depth"],
                        "time": f"{obj.get('simplification_time', 0.0):.8f}"
                }

                all_results.append(combined)

        finally:
            for _, fp in fps:
                fp.close()

        # Write a single proper JSON file
        with open(outfile, "w") as out_f:
            json.dump(all_results, out_f, indent=2)

if __name__ == '__main__':
    main()
