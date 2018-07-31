/* This scripts receives chromosomal regions and extracts genes which exist within these regions.

Usage: extract_genes [Gene_file_path][Input_file_path] [Output_file_path]


Parameter

Gene_file_path: String
path for gene list BED file
download preferable gene list file from UCSC table brouser (ex: hg19/hg38, gencode/refseq)

Input_file_path: String
path for BED file which contains [chrom, chromStart, and chromEnd] in each line (with or without a header)
reference genome version must be the same as the gene list file
ex) chrX    100000  150000

Output_file_path: String
path for output file
each line contains one gene ID

*/

#[allow(unused_imports)]
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::io::prelude::*;
#[allow(unused_imports)]
use std::io::{stdin, stdout, BufReader, BufWriter, Write};

static BUF_SIZE: usize = 10 * 1024 * 1024;
type GRange = (usize, usize);

fn main() {
    let (gene_file, input, output) = get_param();
    // open gene list file
    let f = File::open(gene_file).unwrap();
    let mut reader = BufReader::with_capacity(BUF_SIZE, f);
    // hash for mapping chromosome name to the index which corresponds to it
    let mut chrom_to_index: HashMap<String, usize> = HashMap::new();
    // chromosome gene lists
    let mut gene_lists: Vec<Vec<(String, GRange)>> = Vec::new();
    // read a file and process each line
    let mut line = String::with_capacity(500);
    let _first = reader.read_line(&mut line).unwrap();
    line.clear();
    while reader.read_line(&mut line).unwrap() > 0 {
        {
            // split line with whitespace
            let vec: Vec<&str> = line.trim().split_whitespace().collect();
            // push (gene_name, region) tuple to the matched chromosome gene list
            let gene_name = vec[1].to_string();
            let region = (
                vec[4].parse::<usize>().unwrap(),
                vec[5].parse::<usize>().unwrap(),
            );
            if let Some(&index) = chrom_to_index.get(vec[2]) {
                gene_lists[index].push((gene_name, region));
            } else {
                gene_lists.push(vec![(gene_name, region)]);
                chrom_to_index.insert(vec[2].to_string(), gene_lists.len() - 1);
            }
        }
        line.clear();
    }
    drop(reader);

    // open input file
    let f = File::open(input).unwrap();
    let mut reader = BufReader::with_capacity(BUF_SIZE, f);
    // list for genes which exist within query regions
    let mut contained_genes = Vec::new();
    // read a file and process by the line
    let mut line = String::with_capacity(100);
    while reader.read_line(&mut line).unwrap() > 0 {
        {
            // split line with whitespace
            let vec: Vec<&str> = line.trim().split_whitespace().collect();
            // check whether each gene exists within the query region
            if let Some(&index) = chrom_to_index.get(vec[0]) {
                let (start, end) = (
                    vec[1].parse::<usize>().unwrap(),
                    vec[2].parse::<usize>().unwrap(),
                );
                for &(ref name, ref range) in gene_lists[index].iter() {
                    if range.1 >= start && range.0 <= end {
                        contained_genes.push(name.clone());
                    }
                }
            }
        }
        line.clear();
    }
    drop(reader);
    // delete redundant elements
    contained_genes.sort();
    contained_genes.dedup();

    // write the result to output file
    let write_f = File::create(output).unwrap();
    let mut writer = BufWriter::with_capacity(BUF_SIZE, write_f);
    for gene in contained_genes.iter() {
        writeln!(&mut writer, "{}", gene).unwrap();
    }
    writer.flush().unwrap();
}

fn get_param() -> (String, String, String) {
    let mut args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        panic!("invalid arguments.");
    }
    let output = args.pop().unwrap();
    let input = args.pop().unwrap();
    let gene = args.pop().unwrap();
    return (gene, input, output);
}
