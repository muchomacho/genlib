#!/bin/bash

# This script makes fasta file which only contains chromosome sequence from human reference fasta file
# Usage: ./make_chrom_fasta.sh [-r reference_fasta_file] [-o output_fasta_file]
shopt -s extglob

while getopts "r:o:" opt; do
    case $opt in
	r) ref=$OPTARG ;;
    o) out=$OPTARG ;;
	[?]) echo "Usage: ${0##*/} [-r reference fasta file] [-o output fasta file]";exit ;;
    esac
done

## compile rust file
rustc -O $( dirname ${0} )/split_fasta.rs
if [$? -gt 0]; then
    echo "*** Compilation failed ***"
    exit 1
fi

mkdir .tmp
split_fasta ${ref} .tmp
if [$? -gt 0]; then
    echo "*** Error occurred at spliting fasta file ***"
    rm split_fasta
    rm -r .tmp
    exit 1
fi

for num in $( seq 22 ) "X" "Y"; do
    cat .tmp/chr${num}.fasta >> ${out}
    if [$? -gt 0]; then
        echo "*** Error occurred at merging chromosome files"
        rm split_fasta
        rm -r .tmp
        exit 1
done

rm split_fasta
rm -r .tmp
