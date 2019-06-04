use std::error::Error;
use std::io::Read;
use std::fs::File;

use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::time::Instant;


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
    let mut f = File::open(conf.dictionary)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let start = Instant::now();
    let results = search(&conf.hash, &contents);
    let duration = start.elapsed();

    print!("Password has been found: {}", results);
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    Ok(())
}

pub fn search(hash: &String, contents: &str) -> String {

    for line  in contents.lines() {
        let mut hasher = Sha1::new();
        hasher.input_str(line);
        let hex = hasher.result_str();
        if hex == hash.clone() {

            return line.parse().unwrap();
        }
    }
    String::from("Plaintext password could not be found.")
}

