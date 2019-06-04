use std::error::Error;
use std::io::{BufReader, BufRead};
use std::fs::File;
use rayon::prelude::*;

use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::time::Instant;
use std::path::Path;


pub struct Config {
    pub dictionary: String,
    pub hash: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let dictionary = args[1].clone();
        let hash = args[2].clone();

        Ok(Config { dictionary, hash})
    }
}

pub fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    let lines = lines_from_file(conf.dictionary);
    let start = Instant::now();
    let results = search(&conf.hash, lines);
    let duration = start.elapsed();

    if results.is_empty() {
        println!("Password is not in dictionary.");
    } else {
        println!("Password has been found: {:?}", results);
    }

    println!("Time Taken: {:?}", duration);
    Ok(())
}

pub fn search(hash: &String, contents: Vec<String>) -> Vec<String> {
    let hash = hash.clone();

    let result: Vec<_> =  contents.into_par_iter()
        .filter(|s| get_sha1_hash(&string_to_static_str(s.to_string())) == hash)
        .collect();
    result
}

pub fn get_sha1_hash(input: &&str) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(input);
    hasher.result_str()
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

