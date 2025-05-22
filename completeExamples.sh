#!/bin/bash

# Limited by rules
for file in Examples/MyExamples/*.p; do
    base=$(basename "$file")
    name="${base%.*}"
    mkdir -p "Examples/MyExamples/Results/$name"
    output_file="Examples/MyExamples/Results/$name/$name"

    for i in 100 500 1000 5000; do
        for j in 1 10; do
            if [[ $j == 10 && ( $i == 1000 || $i == 5000 ) ]]; then
                continue
            fi
            start=$(date +%s.%N)
            if ! timeout 10800s twee --complete --max-rules "$i" --random-mode-best-of "$j" "$file" > "${output_file}${i}RulesBestOf${j}" ; then
                echo -e "\nTimeout after 3 hours\n" >> "${output_file}${i}RulesBestOf${j}"
            fi
            end=$(date +%s.%N)
            echo -e "\nExecution time: $(awk "BEGIN {print $end - $start}") seconds\n" >> "${output_file}${i}RulesBestOf${j}"
        done
    done

    # Limited by term size
    for i in 3 5 8; do
        start=$(date +%s.%N)
        if ! timeout 10800s twee --complete --max-term-size "$i" "$file" > "${output_file}${i}TermSize" ; then
            echo -e "\nTimeout after 3 hours\n" >> "${output_file}${i}TermSize"
        fi
        end=$(date +%s.%N)
        echo -e "\nExecution time: $(awk "BEGIN {print $end - $start}") seconds\n" >> "${output_file}${i}TermSize"
    done

    # Limited by CP depth
    for i in 1 2 3; do
        start=$(date +%s.%N)
        if ! timeout 10800s twee --complete --max-cp-depth "$i" "$file" > "${output_file}${i}CPDepth"; then
            echo -e "\nTimeout after 3 hours\n" >> "${output_file}${i}CPDepth"
        fi
        end=$(date +%s.%N)
        echo -e "\nExecution time: $(awk "BEGIN {print $end - $start}") seconds\n" >> "${output_file}${i}CPDepth"
    done

    # Complete subsets
    start=$(date +%s.%N)
    if ! timeout 10800s twee --complete --max-rules 500 --complete-subsets "$file" > "${output_file}CompleteSubsets"; then
        echo -e "\nTimeout after 3 hours\n" >> "${output_file}CompleteSubsets"
    fi
    end=$(date +%s.%N)
    echo -e "\nExecution time: $(awk "BEGIN {print $end - $start}") seconds\n" >> "${output_file}CompleteSubsets"
done
