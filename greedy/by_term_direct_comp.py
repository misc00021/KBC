import argparse
import json
import os
import sys

def size(term):
    clean = term.replace('(', ' ').replace('"', ' ').replace(')', ' ')
    return len(clean.split())

# Compare results of two rulesets/methods by term
def main():
    parser = argparse.ArgumentParser(description="Compare results of two rulesets by term, sorted with ascending speedup.")
    parser.add_argument("Baseline", type=str, help="Results when using base rules.")
    parser.add_argument("Extended", type=str, help="Results when using extended rules.")

    args = parser.parse_args()
    base_file = args.Baseline
    ext_file = args.Extended

    ext_name = os.path.splitext(os.path.basename(ext_file))[0]
    outfile = os.path.join(
        "results/basecomps/",
        f"by_term_direct_comp_{ext_name}.json")
    os.makedirs(os.path.dirname(outfile), exist_ok=True)
    all_results = []

    with open(base_file, "r") as fbase, open(ext_file, "r") as fext:
        for line_base, line_ext in zip(fbase, fext):
            obj_base = json.loads(line_base)
            obj_ext = json.loads(line_ext)

            # Take base info from the first file
            base_term = obj_base["original_term"]

            combined = {
                "base_term": base_term,
                "Baseline": {
                    "output_term": obj_base["simplified_term"],
                    "output_size": size(obj_base["simplified_term"])
                },
                "Extended": {
                    "output_term": obj_ext["rewritten_term"],
                    "output_size": size(obj_ext["rewritten_term"])
                },
                "size_diff": (size(obj_base["simplified_term"]) - size(obj_ext["rewritten_term"])),
            }
            all_results.append(combined)
    all_results.sort(key=lambda x: x["size_diff"])
    with open(outfile, "w") as outf:
        json.dump(all_results, outf, indent=2)
    print(f"Written to {outfile}")





if __name__ == '__main__':
    main()
