extern crate byteorder;
extern crate pbr;

mod data;
mod bsa;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

use data::BSAParser;

fn main() {

    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Please supply .bsa file in command call,");
        return;
    }
    if args.len() < 3 {
        println!("Please supply output directory path in command call.");
        return;
    }
    let path = &args[1];
    let output = &args[2];
    let cwd = env::current_dir().unwrap();
        let file_opt = File::open(path);
        match file_opt {
            Ok(file) => {

                let start = SystemTime::now();
                let mut reader = BufReader::new(file);
                let mut buf: Vec<u8> = vec![];
                reader.read_to_end(&mut buf);

                println!("Parsing BSA file...");
                let mut parser = BSAParser::new(buf);
                let bsa_opt = parser.parse();

                if !bsa_opt.is_none() {
                    let end = SystemTime::now();
                    let duration = end.duration_since(start)
                        .expect("Time backwards");
                    let in_ms = duration.as_secs() * 1000 + duration.subsec_nanos() as u64 / 1_000_000;
                    println!("BSA file parsed, took {:?}ms", in_ms);
                    let bsa = bsa_opt.unwrap();
                    println!("Exporting BSA file...");
                    bsa.export(output.to_string());
                    println!("Done! BSA successfully exported.");
                }

            },
            Err(e) => {
                println!("Error opening file: {:?}", e);
            }
        }

}

