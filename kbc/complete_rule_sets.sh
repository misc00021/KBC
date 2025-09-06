#!/bin/bash

for file in base_rules/as_egg/seperate_division/*; do
    for i in 40 60 80 100 150 200; do
        echo "Running with $i rules on $file"
        /home/michi/Documents/thesis/KBC/kbc/target/debug/kbc "$file" --num_rules="$i"
    done
    for i in 3 4 6 8; do
        echo "Running with max term size $i on $file"
        /home/michi/Documents/thesis/KBC/kbc/target/debug/kbc "$file" --max_term_size="$i"
    done
done
