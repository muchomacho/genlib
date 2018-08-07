#!/bin/bash

shopt -s extglob

# exit function
function error_exit() {
    echo "*** Error occurred at $2 "
    rm -r $1
    exit 1
}

while getopts "d:r:" opt; do
    case $opt in
    d) dir=$OPTARG ;;
    o) out=$OPTARG ;;
	r) ref=$OPTARG ;;
	[?]) echo "Usage: ${0##*/} [-d sequence file directory] [-o output bam file] [-r reference fasta file]";exit ;;
    esac
done

## merge sequenced files of multiple lanes
tmp=".tmp"
mkdir ${tmp}
for file in $( ls ${dir}/*_L001* ); do
    merged_file="${tmp}/${file/_L001/}"
    cp ${file} ${merged_file}
    for lane_file in ${file/L001/L*}; do 
        cat ${lane_file} >> ${merged_file}
    done
    if [ $? -gt 0 ]; then
        error_exit ${tmp} "merging same lane file"
    fi
done

## remove adapter sequence
counter=1
for file in $( ls ${tmp}/*R1* ); do
    NGmerge -1 ${file} -2 ${file/R1/R2} -o ${tmp}/$((counter++))
    if [ $? -gt 0 ]; then
        error_exit ${tmp} "removing adapter"
    fi
done

## mapping reads
for file in $( ls ${tmp}/*_1* ); do
    bwa mem -t 32 ${ref} ${file} ${file/_1/_2} > ${file/_1/}.sam
    if [ $? -gt 0 ]; then
        error_exit ${tmp} "bwa mapping"
    fi
done

## convert samfile into sorted bamfile
for file in $( ls ${tmp}/*.sam ); do
    samtools-1.3.1 view -Su ${file} | samtools-1.3.1 sort - > ${file/sam/bam}
    if [ $? -gt 0 ]; then
        error_exit ${tmp} "converting samfile into bamfile"
    fi
done

## merge bamfiles
ls ${tmp}/*.bam > ${tmp}/bam_list.txt
samtools-1.3.1 merge -b ${tmp}/bam_list.txt ${out}
if [ $? -gt 0 ]; then
    error_exit ${tmp} "merging bamfile"
fi

## convert bamfile into bedfile
bedtools bamtobed -i ${output} > ${out/bam/bed}
if [ $? -gt 0 ]; then
    error_exit ${tmp} "converting bamfile to bedfile"
fi

echo "*** Pipeline successfully finished. ***"
rm -r ${tmp}


    