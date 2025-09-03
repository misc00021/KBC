#!/bin/bash

for file in /home/michi/Documents/thesis/KBC/term_gen/no_div_sets/*.txt; do
    echo "Running on $file"
    time target/debug/kbc-test "$file"
done
