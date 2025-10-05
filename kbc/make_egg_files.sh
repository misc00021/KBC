#!/bin/bash

# Hardcoded parent folder containing subfolders
PARENT_DIR="rule_collections/collections_for_EqSat/replacing_sets"

# Path to your Python script
PY_SCRIPT="make_egg_file.py"

# Iterate over all subdirectories
for dir in "$PARENT_DIR"/*/ ; do
    if [ -d "$dir" ]; then
        echo "Processing folder: $dir"
        python3 "$PY_SCRIPT" "$dir"
    fi
done