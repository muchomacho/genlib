#[allow(unused_imports)]
use std::io::{stdin, stdout, Write, BufReader, BufWriter};
#[allow(unused_imports)]
use std::io::prelude::*;
#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::env;

// This script divides one fasta file into multiple files by the sequence
// Used to create a fasta file which only contains 24 chromosome information

static BUF_SIZE: usize = 1024 * 1024 * 10;

fn main() {
    // open target fasta file
    let input = get_param();
    let f = File::open(input).unwrap();
    let mut reader = BufReader::with_capacity(BUF_SIZE, f);

    // read file by the line
    let mut bytes;
    let mut line = Vec::new();
    bytes = reader.read_until(b'\n', &mut line).unwrap();
    if line[0] != b'>' {
        panic!("invalid comment line");
    }
    'outer: loop {
        // modify comment line so that it can become a valid file name
        // delete delimiters at both side
        if bytes <= 2 {
        panic!("invalid comment line");
        }
        let mut del_indices = Vec::new();
        for i in 1..(line.len() - 1) {
            if is_delim(line[i]) {
                del_indices.push(i);
            } else {
                break;
            }
        } 
        for i in (1..(line.len() - 1)).rev() {
            if is_delim(line[i]) {
                del_indices.push(i);
            } else {
                break;
            }
        }
        del_indices.sort();
        del_indices.dedup();
        del_indices.reverse();
        for &i in del_indices.iter() {
            line.remove(i);
        }
        if line.len() <= 2 {
            panic!("invalid comment line");
        }
        // transform internal delimiters into '_' 
        for i in 1..(line.len() - 1) {
            if is_delim(line[i]) {
                line[i] = b'_';
            }
        }

        // create a file and write sequence
        let write_f = File::create(String::from_utf8_lossy(&line[1..(line.len() - 1)]).to_string() + ".fasta").unwrap();
        let mut writer = BufWriter::with_capacity(BUF_SIZE, write_f);
        writer.write(&line[..]).unwrap();
        line.clear();
        loop {
            bytes = reader.read_until(b'\n', &mut line).unwrap();
            // EOF of input file
            if bytes == 0 {
                let _ok = writer.flush().unwrap();
                break 'outer;
            }
            // new sequence
            if line[0] == b'>' {
                let _ok = writer.flush().unwrap();
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

fn is_delim(b: u8) -> bool { b == b'\r' || b == b'\t' || b == b' ' || b == b';' }
