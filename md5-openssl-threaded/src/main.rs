use openssl::hash::{hash, MessageDigest};
use std::{thread, time::Duration};
use hex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        std::println!("Missing Arguments");
        std::process::exit(-1);
    }
    let name = &args[1];
    let file_input = File::open(name).expect("Can't open file!");
    let mut file_output = File::create(name.to_owned() + ".md5").expect("Can't Open file_output");
    let reader = BufReader::new(file_input);

    

    for line in reader.lines() {
        if let Ok(line) = line {
            let res = hash(MessageDigest::md5(), line.as_bytes()).unwrap();
            writeln!(&mut file_output, "{}", hex::encode(res)).expect("Can't write in file");
        }
    }
}
