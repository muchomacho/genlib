#[allow(unused_imports)]
use std::io::{stdin, stdout, Write, BufReader, BufWriter};
#[allow(unused_imports)]
use std::io::prelude::*;
#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::env;

// This script divides one fasta file into multiple files by headers
// Used to create a fasta file which only contains 24 chromosome information

fn main() {
    let input = get_param();
    let f = File::open(input).unwrap();
    let mut reader = BufReader::new(f);

    let mut bytes;
    let mut line = Vec::new();
    bytes = reader.read_until(b'\n', &mut line).unwrap();
    if bytes == 0 || line[0] != b'<' {
        panic!("input file is not a fasta file.");
    }
    'outer: loop {
        let write_f = File::create(String::from_utf8_lossy(&line[1..(line.len() - 1)]).to_string() + ".fasta").unwrap();
        let mut writer = BufWriter::new(write_f);
        writer.write(&line[..]).unwrap();
        line.clear();
        loop {
            bytes = reader.read_until(b'\n', &mut line).unwrap();
            if bytes == 0 {
                break 'outer;
            }
            if line[0] == b'<' {
                continue 'outer;
            }
            writer.write(&line[..]).unwrap();
            line.clear();
        }
    }
}

fn get_param() -> String {
    let args: Vec<String> = env::args().collect();
    return args.into_iter().nth(1).unwrap();
}
