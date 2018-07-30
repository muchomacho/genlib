#!/bin/bash

shopt -s extglob

while getopts "d:r:" opt; do
    case $opt in
    d) dir=$OPTARG ;;
	r) reference_fasta_file=$OPTARG ;;
	[?]) echo "Usage: ${0##*/} [-d sequence file directory] [-r reference_fasta_file] ";exit ;;
    esac
done

cd ${dir}
mkdir _tmp
for file in $( ls *_*_L001_* ); do
    cat ${file} $( echo ${file} | sed s/L001/L002/ ) > _tmp/$( echo ${file} | sed s/_L001// )
done

cd _tmp
for file in $( ls *_*_R1_* ); do
    ~/bwa/bwa mem -t 32 reference_fasta_file ${file} $( echo ${file} | sed s/R1/R2/ ) > $( echo ${file} | sed s/_R1// ).sam
done

for file in $( ls *.sam ); do
    samtools-1.3.1 view -Su ${file} | samtools-1.3.1 sort - > $( echo ${file} | sed s/sam/bam/ )
done

ls *.bam > bam_list.txt
samtools-1.3.1 merge -b bam_list.txt ../merged.bam

cd ..
bedtools bamtobed -i merged.bam > merged.bed

rm -r _tmp


    