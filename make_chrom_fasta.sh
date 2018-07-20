#!/bin/bash

# This script makes fasta file which only contains chromosome sequence from human reference fasta file
# Usage: ./make_chrom_fasta.sh [-r reference_fasta_file] [-o output_fasta_file]
shopt -s extglob

while getopts "r:o:" opt; do
    case $opt in
	r) REF=$OPTARG ;;
    o) OUT=$OPTARG ;;
	[?]) echo "Usage: ${0##*/} [-r reference_fasta_file] [-o output_fasta_file]";exit ;;
    esac
done

SRC_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"
REF_DIR="$( cd "$( dirname "${REF}" )" >/dev/null && pwd )"
OUT_DIR="$( cd "$( dirname "${OUT}" )" >/dev/null && pwd )"

cd $SRC_DIR
rustc -O split_fasta.rs
mkdir _tmpdir
cd _tmpdir
../split_fasta ${REF_DIR}/$( basename reference_fasta_file )

for num in $( seq 22 ) "X" "Y"
do
    cat chr${num}.fasta >> ${OUT_DIR}/$( basename output_fasta_file )
done

cd ..
rm split_fasta
rm -r _tmpdir
