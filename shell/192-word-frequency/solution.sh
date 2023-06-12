#!/bin/bash

LC_ALL=C 

array=( $(<words.txt) )

sorted=$(printf '%s\n' "${array[@]}" | sort -r)

printf '%s\n' "${sorted[@]}" | uniq -c | sort -k1 -r | awk '{print $2, $1}'