#!bin/sh

# This script extracts genomic regions of target genes by bed format
# output: standard output
# Usage: get_gene_regions.sh <gene count file> <gene bed file> 

grep -f $1 $2 | awk -F'\t' '{print $3 "\t" $5 "\t" $6 }'
