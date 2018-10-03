
mod data;
mod bsa;

use std::env;
use std::fs::File;

fn main() {

    let mut args = env::args();
    let path = args.nth(1);
    let cwd = env::current_dir().unwrap();

    if path == None {
        println!("Please supply ")
    } else {
        let file_opt = File::open(path.unwrap());
        match file_opt {
            Ok(file) => {

            },
            Err(e) => {
                println!("Error opening file: {:?}", e);
            }
        }
    }
}

