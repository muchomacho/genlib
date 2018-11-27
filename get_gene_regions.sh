#!/bin/bash

# This script extracts genomic regions of target genes by bed format
# output: standard output
# Usage: get_gene_regions.sh <gene list file> <gene bed file> 

grep -w -f $1 $2 | awk -F'\t' '{print $3 "\t" $5 "\t" $6 }'
