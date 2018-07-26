#[allow(unused_imports)]
use std::io::{stdin, stdout, Write, BufReader, BufWriter};
#[allow(unused_imports)]
use std::io::prelude::*;
#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::env;

static BUF_SIZE: usize = 1024 * 1024 * 10;
type GRange = (usize, usize);

// This scripts accepts chromosomal regions and extracts genes which exist within these regions.
//
// Usage: ./extract_genes [gene_file_path][input_file_path] [output_file_path]
//
// gene_file_path: String
// path for gene list BED file
// download preferable gene list file from UCSC table brouser (hg19/hg38, gencode/refseq)
//
// input_file_path: String
// path for BED file which contains [chrom, chromStart, and chromEnd] in each line (with or without a header)
// reference genome version must be the same as the gene list file
// ex) chrX    100000  150000
//
// output_file_path: String
// path for output file
// each line contains one gene ID

// Todo: 
// revise read_to_string error handling

fn main() {
    let (gene_file, input, output) = get_param();
    // open gene list file
    let mut gene_f = File::open(gene_file).unwrap();
    // chromosome gene lists 
    let mut buffer = String::new();
    let _bytes = gene_f.read_to_string(&mut buffer).unwrap();
    let mut gene_lists: Vec<Vec<(String, GRange)>> = vec![Vec::new(); 24];
    // read a file and process each line
    for line in buffer.lines() {
        // split line with '\t'
        let vec: Vec<&str> = line
            .trim()
            .split('\t')
            .map(|s| s.trim())
            .collect();
        // push (gene_name, region) tuple to the matched chromosome gene list
        let gene_name = vec[1].to_string(); 
        let region = (vec[4].parse::<usize>().unwrap(), vec[5].parse::<usize>().unwrap());
        let chrom: &str = vec[2].split("chr").nth(1).unwrap();
        match chrom {
            "X" => gene_lists[22].push((gene_name, region)),
            "Y" => gene_lists[23].push((gene_name, region)),
            _ => if let Ok(num) = chrom.parse::<usize>() { gene_lists[num - 1].push((gene_name, region)) },
        };
    }
    drop(buffer);

    // open input file
    let mut input_f = File::open(input).unwrap();
    // list for genes which exist within query regions
    let mut contained_genes = Vec::new();
    // read a file and process by the line
    let mut buffer = String::new();
    let _bytes = input_f.read_to_string(&mut buffer).unwrap();
    for line in buffer.lines() {
        // split line with '\t'
        let vec: Vec<&str> = line
            .trim()
            .split('\t')
            .map(|s| s.trim())
            .collect();
        // check whether each gene exists within the query region
        let chrom = vec[0].split("chr").nth(1).unwrap();
        let mut num = 24;
        match chrom {
            "X" => num = 22,
            "Y" => num = 23,
            _ => if let Ok(n) = vec[0].parse::<usize>() { num = n - 1 },
        };
        if num == 24 {
            continue;
        }
        let (start, end) = (vec[1].parse::<usize>().unwrap(), vec[2].parse::<usize>().unwrap());        
        for &(ref name, ref range) in gene_lists[num].iter() {
            if range.1 >= start && range.0 <= end {
                contained_genes.push(name.clone());
            }
        }
    }
    // delete redundant elements
    contained_genes.sort();
    contained_genes.dedup();
    drop(buffer);

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

