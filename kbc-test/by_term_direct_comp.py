import argparse
import json
import os
import sys

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
            input_weight_simple = obj_base["input_weight_simple"]
            input_weight_complex = obj_base["input_weight_complex"]
            input_depth = obj_base["input_depth"]

            combined = {
                "base_term": base_term,
                "input_weight_simple": input_weight_simple,
                "input_weight_complex": input_weight_complex,
                "input_depth": input_depth,
                "Baseline": {
                    "iterations": obj_base.get("iterations", 0),
                    "stop_reason": obj_base.get("stop_reason", "unknown"),
                    "output_term": obj_base["simplified_term"],
                    "output_weight_simple": obj_base["output_weight_simple"],
                    "output_weight_complex": obj_base["output_weight_complex"],
                    "output_depth": obj_base["output_depth"],
                    "simplification_time": obj_base.get("simplification_time", 0.0)
                },
                "Extended": {
                    "iterations": obj_ext.get("iterations", 0),
                    "stop_reason": obj_ext.get("stop_reason", "unknown"),
                    "output_term": obj_ext["simplified_term"],
                    "output_weight_simple": obj_ext["output_weight_simple"],
                    "output_weight_complex": obj_ext["output_weight_complex"],
                    "output_depth": obj_ext["output_depth"],
                    "simplification_time": obj_ext.get("simplification_time", 0.0)
                },
                "speedup": (obj_base.get("simplification_time", 0.0) / obj_ext.get("simplification_time", 1.0)),
                "weight_simple_diff": (obj_base["output_weight_simple"] - obj_ext["output_weight_simple"]),
                "weight_complex_diff": (obj_base["output_weight_complex"] - obj_ext["output_weight_complex"]),
                "depth_diff": (obj_base["output_depth"] - obj_ext["output_depth"])
            }
            all_results.append(combined)
    all_results.sort(key=lambda x: x["speedup"])
    with open(outfile, "w") as outf:
        json.dump(all_results, outf, indent=2)





if __name__ == '__main__':
    main()
