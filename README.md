# Content
- ## kbc (Rule Set Completion)
    ### Usage
    - target/debug/kbc <input_file> --num_rules=<num_rules>         to complete until <num_rules> is reached
    - target/debug/kbc <input_file> --max_term_size=<max_size>      to complete until the next rule's LHS has len > max_size
    - target/debug/kbc <input_file> --add_guards                    to add constraints, preventing division by zero
    ### Contains hardcoded flags for...
    - handling rules, unorderable using KBO
    - extending/replacing rules during completion
- ## kbc-test (EqSat Tests)
    - Contains code for testing with EqSat using egg
    - Contains Python scripts for comparing and summarizing results
- ## term_gen
    - Randomly generates test terms
- ## greedy
    - Implements greedy rewriting
    - Contains Python script to compare results of two rulesets/methods
    - Contains Python script to run on a specific directory and summarize the results as csv

# Notes for replication of results
- Due to condition encoding, Twee may generate nonsensical rules, e.g.: 0 -> 1 if is_not_zero(0), 0 -> 0/x if is_not_zero(x)
    - egg will complain, rules need to be deleted manually
        - Maybe works automatically now
- egg math has no negation operator, -1 is a constant
    - Using -1 as a constant messes with Twees ordering => program kbc changes -1 to (neg 1) before completion and back after