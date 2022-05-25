use reqwest;
use std::io::Read;

fn main() {
    let body = reqwest::get("https://www.rust-lang.org").text();

    println!("body = {:?}", body);
}"