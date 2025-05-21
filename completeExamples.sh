#!/bin/bash

#limited by rules
for file in Examples/MyExamples/*.p; do
        for i in 100 500 1000 50000; do
            for j in 1 10; do
                start=$(date +%s.%N)
                twee --complete --max-rules $i --random-mode-best-of $j "$file" > "${file%.*}${i}RulesBestOf${j}";
                end=$(date +%s.%N)
                echo -e "\nExecution time: $(awk "BEGIN {print $end - $start}") seconds\n" >> "${file%.*}${i}RulesBestOf${j}";
                done
            done
        done

#limited by term size
for file in Examples/MyExamples/*.p; do
        for i in 3 5 8; do
            start=$(date +%s.%N)
            twee --complete --max-term-size $i "$file" > "${file%.*}${i}TermSize";
            end=$(date +%s.%N)
            echo -e "\nExecution time: $(awk "BEGIN {print $end - $start}") seconds\n" >> "${file%.*}${i}TermSize";
            done
        done

#limited by CP depth
for file in Examples/MyExamples/*.p; do
        for i in 3 5 8; do
            start=$(date +%s.%N)
            twee --complete --max-cp-depth $i "$file" > "${file%.*}${i}CPDepth";
            end=$(date +%s.%N)
            echo -e "\nExecution time: $(awk "BEGIN {print $end - $start}") seconds\n" >> "${file%.*}${i}CPDepth";
            done
        done

#complete subsets
for file in Examples/MyExamples/*.p; do
        start=$(date +%s.%N)
        twee --complete --max-rules 500 --complete-subsets "$file" > "${file%.*}CompleteSubsets";
        end=$(date +%s.%N)
        echo -e "\nExecution time: $(awk "BEGIN {print $end - $start}") seconds\n" >> "${file%.*}CompleteSubsets";
        done