#!/bin/bash

for file in /home/michi/Documents/thesis/KBC/term_gen/sets/*.txt; do
    echo "Running on $file"
    time target/release/kbc-test "$file"
done
shutdown -h now
