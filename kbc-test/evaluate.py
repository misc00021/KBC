import os
import json

def symCount(term: str):
    clean = term.replace('(', ' ').replace('"', ' ').replace(')', ' ')
    return len(term.split())

def main():
    path = "all_results/"
    ruleTestDict = {}
    results = {}
    simplifiable = {}
    for file in os.listdir(path):
        name = (file.split(".")[0] + '.' + file.split(".")[1]).removesuffix(".jsonl")
        (_, ruleSet, testSet, timeLimit) = name.split("-")
        if (ruleSet, testSet) not in ruleTestDict:
            ruleTestDict[(ruleSet, testSet)] = []
        ruleTestDict[(ruleSet, testSet)].append(timeLimit)
    for name in ["summary_results", "only_simplifiable"]:
        for (ruleSet, testSet) in ruleTestDict:
            timeLimits = ruleTestDict[(ruleSet, testSet)]
            averageResults = {}
            unavailable = []
            for limit in timeLimits:
                pathToFile = f"EqSat-{ruleSet}-{testSet}-{limit}.jsonl"
                with open(path + pathToFile, "r") as f:
                    lines = f.readlines()
                    if "small" in testSet and len(lines) < 2951:
                        unavailable.append(pathToFile)
                        continue
                    elif "medium" in testSet and len(lines) < 406:
                        unavailable.append(pathToFile)
                        continue
                    elif  "large" in testSet and len(lines) < 117:
                        unavailable.append(pathToFile)
                        continue
                    elif "huge" in testSet and len(lines) < 248:
                        unavailable.append(pathToFile)
                        continue
                    sumDiff = 0.0
                    complexSumDiff = 0.0
                    count = 0
                    id = 0
                    for line in lines:
                        if name == "only_simplifiable" and testSet in simplifiable and id not in simplifiable[testSet]:
                            id += 1
                            continue
                        obj = json.loads(line)
                        if "error" in obj:
                            unavailable.append(pathToFile)
                            break
                        input_weight_simple = obj["input_weight_simple"]
                        input_weight_complex = obj["input_weight_complex"]
                        input_depth = obj["input_depth"]
                        out_simple = obj["output_weight_simple"]
                        out_complex = obj["output_weight_complex"]
                        out_depth = obj["output_depth"]
                        inputTerm = obj["original_term"]
                        outputTerm = obj["simplified_term"]
                        sumDiff += symCount(inputTerm) - symCount(outputTerm)
                        if name == "summary_results" and sumDiff > 0:
                            if testSet not in simplifiable:
                                simplifiable[testSet] = set()
                            simplifiable[testSet].add(count)
                        count += 1
                        id += 1
                    averageResults[limit] = sumDiff / count if count > 0 else 0.0
                    results[(ruleSet, testSet)] = averageResults
        # Dump results to JSON file
        with open(f"results/eval/{name}.jsonl", "w") as out_file:
            for (ruleSet, testSet), averages in results.items():
                summary = {
                    "ruleSet": ruleSet,
                    "testSet": testSet,
                    "averages": averages
                }
                out_file.write(json.dumps(summary) + "\n")
        #Dump results to CSV file
        with open(f"results/eval/{name}.csv", "w") as out_file:
            out_file.write("ruleSet,testSet," + ",".join(sorted(set().union(*[set(l.keys()) for l in results.values()]))) + "\n")
            for (ruleSet, testSet), averages in results.items():
                out_file.write(f"{ruleSet},{testSet}," + ",".join(str(averages.get(limit, 0.0)) for limit in sorted(averages.keys())) + "\n")
                
                    

if __name__ == "__main__":
    main()