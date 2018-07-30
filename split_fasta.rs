/* 

This script divides one fasta file into multiple files by the sequence
Used to create a fasta file which only contains 24 chromosome information
Usage: split_fasta [input FASTA file]

*/

#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::io::prelude::BufRead;
#[allow(unused_imports)]
use std::io::{stdin, stdout, BufReader, BufWriter, Write};

// read/write buffer size
static BUF_SIZE: usize = 1024 * 1024 * 10;
// buffer size for line
// assume that only ASCII characters are contained in the input FASTA file
static LINE_SIZE: usize = 150;

fn main() {
    // open target fasta file
    let input = get_param();
    let f = File::open(input).unwrap();
    let mut reader = BufReader::with_capacity(BUF_SIZE, f);

    // read file by the line
    let mut num_bytes;
    let mut line = Vec::with_capacity(LINE_SIZE);
    num_bytes = reader.read_until(b'\n', &mut line).unwrap();
    if line[0] != b'<' {
        panic!("invalid comment line");
    }

    // comment line process
    'outer: loop {
        if num_bytes <= 2 {
            panic!("invalid comment line");
        }
        // modify comment line so that it can become a valid file name
        let mut file_name = String::from_utf8_lossy(&line[..]).into_owned();
        file_name = file_name
            .replace("<", "")
            .trim()
            .replace(" ", "_")
            .replace("\t", "_")
            .replace(";", "_") + ".fasta";
        // create a split FASTA file
        let write_f = File::create(file_name).unwrap();
        let mut writer = BufWriter::with_capacity(BUF_SIZE, write_f);
        // sequence line process
        // read/write a sequence by the line
        writer.write(&line[..]).unwrap();
        line.clear();
        loop {
            num_bytes = reader.read_until(b'\n', &mut line).unwrap();
            // EOF of input file
            if num_bytes == 0 {
                writer.flush().unwrap();
                break 'outer;
            }
            // new comment line
            if line[0] == b'<' {
                writer.flush().unwrap();
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
