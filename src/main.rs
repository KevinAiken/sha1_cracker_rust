use std::{env, process};
use sha1_cracker_rust::Config;

fn main() {
    //example: cargo run passwordlist.txt b7a875fc1ea228b9061041b7cec4bd3c52ab3ce3
    let args: Vec<String> = env::args().collect();

    let conf = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = sha1_cracker_rust::run(conf) {
        println!("Application error: {}", e);
        process::exit(1);
    }

}
