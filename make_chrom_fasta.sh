#!/bin/bash

# This script makes fasta file which only contains chromosome sequence from human reference fasta file
# Usage: ./make_chrom_fasta.sh [-r reference_fasta_file] [-o output_fasta_file]
shopt -s extglob

while getopts "r:o:" opt; do
    case $opt in
	r) reference_fasta_file=$OPTARG ;;
    o) output_fasta_file=$OPTARG ;;
	[?]) echo "Usage: ${0##*/} [-r reference_fasta_file] [-o output_fasta_file]\n";exit "&1" ;;
    esac
done

rustc -O split_fasta.rs
mkdir _tmpdir
cd _tmpdir
../split_fasta reference_fasta_file

for num in `seq 22` "X" "Y"
do
    cat chr${num}.fasta >> output_fasta_file
done

cd ..
rm split_fasta
rm -r tmpdir
