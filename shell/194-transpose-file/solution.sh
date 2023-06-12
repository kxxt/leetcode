#!/bin/bash

width=$(head -n 1 file.txt | wc -w)
height=$(cat file.txt | wc -l)
total=$(( $width * $height ))
declare -a mat
row=0

while read line; do
    read -a array <<< "$line"
    for (( i=0; i<$width; i++ ))
    do
        # row = row, col = i
        idx=$(( $row + $i * $height ))
        mat[$idx]="${array[$i]}"
    done
    ((row++))
done < file.txt

for (( i=0; i<$total; i=i+$height ))
do
    echo "${mat[@]:$i:$height}"
done