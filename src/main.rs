use tree::{find_word, generate_tree};
use std::io::{self, BufReader};
use std::fs::File;
use std::time::Instant;

#[allow(unused)]
use brute_force::*;

mod tree;
mod brute_force;

fn main() {
    let mut scramble = String::new();
    io::stdin().read_line(&mut scramble).unwrap();

    let word_file = BufReader::new(File::open("words.txt").unwrap());

    let tree = generate_tree(word_file);

    // tree approach
    let start = Instant::now();
    let longest = find_word(&tree, &scramble);
    let end = Instant::now();
    
    let length = end.duration_since(start).as_micros();
    println!("{length} micro seconds, answer = {longest}");
}