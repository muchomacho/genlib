// Usage: bed_intersect <BED file1> <BED file2>
// Output: Standard output

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::cmp::{max, min};
use std::collections::{BTreeMap, BTreeSet};
use std::io::{stdin, stdout, BufReader, BufWriter, Write};

static BUFSIZE: usize = 1024;
static LINESIZE: usize = 100;
static UNIT: i64 = 10_000;

fn main() {
    let mut args:Vec<&str> = env::args().collect();
    let (file1, file2) = (args[1], args[2]);
    let (regions1, regions2) = (read_bed(file1), read_bed(file2));
    let (chroms1, chroms2) = (regions1.keys().cloned().collect::BTreeSet<_>(), regions1.keys().cloned().collect::BTreeSet<_>());
    let common_chroms: Vec<_> = chroms1.intersection(&chroms2).cloned().collect();
    let mut results = BTreeMap::new();
    for chrom in common_chroms.iter() {
        let (min_val1, max_val1, intervals1) = regions1.remove(chrom).unwrap();
        let (min_val2, max_val2, intervals2) = regions2.remove(chrom).unwrap();
        let (min_val, max_val) = (min(min_val1, min_val2), max(max_val1, max_val2));
        let mut chrom_region = vec![0 as i64; max_val - min_val + 1];
        let offset = min_val;
        for &(start, end) in regions1.iter() {
            chrom_region[start - offset] += 1;
            chrom_region[end - offset] -= 1;
        }
        for &(start, end) in regions2.iter() {
            chrom_region[start - offset] += UNIT;
            chrom_region[end - offset] -= UNIT;
        }
        let mut intersect = Vec::new();
        let mut counter = 0;
        let mut start = std::usize::MAX;
        for i in chrom_region.len() {
            counter += chrom_region[i];
            if counter > UNIT + 1 {
                if start == std::usize::MAX {
                    start = i;
                }
            } else if start != std::usize::MAX {
                intersect.push((start + offset, i + offset));
                start = std::usize::MAX;
            }
        }
        if start != std::usize::MAX {
            intersect.push((start, chrom_region.len() - 1 + offset));
        }
        results.insert(chrom.to_string(), intersect).unwrap();
    }

    for (chrom, intersect) in results.into_iter() {
        for &(start, end) in intersect.iter() {
            println!("{}\t{}\t{}", chrom, start, end);
        }
    }

}

fn read_bed(file_path: &str) -> BTreeMap<String, (usize, usize, Vec<(usize, usize))>> {
    let f = File::open(file_path).unwrap();
    let mut reader = BufReader::with_capacity(BUFSIZE, f);
    let mut line = String::with_capacity(LINESIZE);
    let mut regions = BTreeMap::new();
    while reader.read_line(&mut line).unwrap() > 0 {
        let vals: Vec<&str> = line.trim().split_whitespace().collect();
        let chrom = vals[0].to_string();
        let start = vals[1].parse::<usize>().unwrap();
        let end = vals[2].parse::<usize>().unwrap();
        if let Some(value) = regions.get_mut(&chrom) {
            value.0 = min(value.0, start);
            value.1 = max(value.1, end);
            value.2.push((start, end));
        } else {
            regions.insert(chrom, (start, end, vec![(start, end)])).unwrap();
        }
    }

    regions
}