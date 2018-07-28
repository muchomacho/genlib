#[allow(unused_imports)]
use std::io::{stdin, stdout, Write, BufReader, BufWriter};
#[allow(unused_imports)]
use std::io::prelude::*;
#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::collections::{VecDeque, HashSet, BTreeSet, BinaryHeap, HashMap};

// read/write buffer size
// input file size is expected to be very big 
static BUF_SIZE: usize = 100 * 1024 * 1024;
// expected size of each line is less than 300 bytes
static LINE_SIZE: usize = 300;

// This script receives sequence reads in BED format and counts the number of reads at given window size
// Usage: ./count_reads [input BED file path] [genome reference type <hg19/hg38>] [MAPQ threshold] [window size] [output dir path]

fn main() {
    // get parameter
    let (src_dir, input_path, reference, q_threshold, window, output_dir) = get_param();
    // open chromosome size file
    let f = if reference == "hg19" { File::open(src_dir + "data/hg19_chrom_size.txt").unwrap() }
            else { File::open(src_dir + "data/hg38_chrom_size.txt").unwrap() };
    let mut reader = BufReader::with_capacity(10 * 1024, f);
    // vector for counting reads
    let mut read_count: Vec<Vec<usize>> = Vec::new();
    // hash for mapping chromosome name to the index which corresponds to it
    let mut chrom_to_index: HashMap<String, usize> = HashMap::new();
    // read chromosome lengths
    let mut index = 0;
    let mut buffer = String::with_capacity(50);
    while reader.read_line(&mut buffer).unwrap() > 0 {
        {
            let elems: Vec<&str> = buffer.trim().split('\t').collect();
            let chrom_name = elems[0].to_string();
            let chrom_length = (elems[1].parse::<usize>().unwrap() / window) + 1;
            chrom_to_index.insert(chrom_name, index);
            read_count.push(vec![0; chrom_length]);
            index += 1;
        }
        buffer.clear();
    }
    drop(reader);

    // open sequence read file
    let f = File::open(input_path).unwrap();
    let mut reader = BufReader::with_capacity(BUF_SIZE, f);
    // read each line and count reads
    // in sequence read file of BED 6/12 format, each line is written as follows
    // [chrom] [chromStart] [chromEnd] [Name] [MAPQ] [Strand] ...
    let mut buffer = String::with_capacity(LINE_SIZE);
    while reader.read_line(&mut buffer).unwrap() > 0 {
        {
            let elems: Vec<&str> = buffer.trim().split_whitespace().collect();
            let chrom_name = elems[0];
            let q = elems[4].parse::<usize>().unwrap();
            // select reads MAPQ >= q_threshold
            if q >= q_threshold {
                // increment count of corresponding window
                let start = elems[1].parse::<usize>().unwrap();
                let end = elems[2].parse::<usize>().unwrap();
                let middle = (start + end) / (2 * window);
                if let Some(&index) = chrom_to_index.get(chrom_name) {
                    read_count[index][middle] += 1;
                }
            }
        }
        buffer.clear();
    }
    drop(reader);

    // output result to file
    for (chrom_name, index) in chrom_to_index.into_iter() {
        // create file in output directory about each chromosome
        let file_path = format!("{}/{}.txt", &output_dir, &chrom_name);
        let f = File::create(file_path).unwrap();
        let mut writer = BufWriter::with_capacity(10 * 1024 , f);
        for &c in read_count[index].iter() {
            writeln!(&mut writer, "{}", c).unwrap();
        }
    }
}

fn get_param() -> (String, String, String, usize, usize, String) {
    let mut params: Vec<String> = env::args().collect();
    if params.len() != 7 {
        panic!("Invalid arguments.");
    }

    let output_dir = params.pop().unwrap();
    let window = params.pop().unwrap().parse::<usize>()
        .expect("Window size must be a positive integer");
    let q_threshold = params.pop().unwrap().parse::<usize>()
        .expect("MAPQ threshold must be a positive integer");
    let reference = params.pop().unwrap();
    if reference != "hg19" && reference != "hg38" {
        panic!("Unknown genome reference");
    }
    let input_path = params.pop().unwrap();
    let mut src_path = params.pop().unwrap();
    while !src_path.ends_with('/') {
        src_path.pop();
    }

    return (src_path, input_path, reference, q_threshold, window, output_dir);
}
