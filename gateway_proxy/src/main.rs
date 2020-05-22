#![feature(proc_macro_hygiene, decl_macro)]

extern crate reqwest;
#[macro_use]
extern crate serde_json;

use std::env;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=functions=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "functions=info");
    }
    pretty_env_logger::init();

    let mut file = File::open("../functions/handler.rs").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let payload = json!({
        "id": 1,
        "language": "rust",
        "code": contents,
        "version": "1.45.0-nightly"
    });
    
    let request_url: String = "http://localhost:3030/functions".to_string();

    let client = reqwest::blocking::Client::new();
    match client
        .post(&request_url)
        .json(&payload)
        .send() {
            Ok(response) => println!("{:?}", response), 
            Err(err) => println!("{:?}", err),
        }
}

fn test_reader_writer() -> std::io::Result<()> {
    let mut file = File::open("functions/handler.rs")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{:?}", contents);

    let mut file_out = File::create("main.rs")?;
    file_out.write_all(contents.as_bytes())?;
    Ok(())
}