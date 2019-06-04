# sha1_cracker_rust

Cracks sha1 hashed passwords with a dictionary.

## Setup
Assuming you have cargo installed, run cargo build --release in the base directory. This will build your executable.

The release flag makes the application around 20 times faster, which will be important for longer dictionaries.

## Usage

After running `$ cargo build --release` you can run the application as follows:

`$ ./target/release/sha1_cracker_rust 10-million-password-list-top-1000000.txt b7a875fc1ea228b9061041b7cec4bd3c52ab3ce3`

This will return 
```
 Password has been found: ["letmein"]
 Time Taken: 72.405423ms
```

The first argument is the dictionary you want to use, and the second is the hash you want to find the plaintext of.

## Motivation
I recently made a similar program in Python for a course which can be found in 
[this repo](https://github.com/KevinAiken/sha1-cracker). I rewrote it in Rust to see 
what kind of performance improvements I could get, and as a fun project to learn a bit of Rust. A direct rewrite of the 
code was a little over twice as fast. The Rayon crate made multithreading very easy, and this version is now  around 10 
times faster than my original Python implementation on my laptop with an 8 core processor. 