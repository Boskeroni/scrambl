use tree::{find_word, generate_tree};
use std::env;
use std::io::BufReader;
use std::fs::File;
use std::time::Instant;

#[allow(unused)]
use brute_force::*;

mod tree;
mod brute_force;

fn main() {
    if env::args().len() == 1 {
        println!("no scramble provided");
        return;
    }
    if env::args().len() > 2 {
        println!("too many arguments provided");
        return;
    }

    let scramble = env::args().nth(1).unwrap();
    let word_file = BufReader::new(File::open("words.txt").unwrap());
    let tree = generate_tree(word_file);

    // tree approach
    let start = Instant::now();
    let longest = find_word(&tree, &scramble);
    let end = Instant::now();
    
    let length = end.duration_since(start).as_micros();
    println!("{length} micro seconds, answer = {longest}");
}