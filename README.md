# Scrambl

A quick solver for "countdown"'s famous word game. The program uses Tries in order to quickly search for words which match the scramble provided.

Due to the overhead of having to construct the Trie of the whole dictionary when running, it is slower than a brute force algorithm (found in the `src/brute_force.rs` file) for single scramble lookups. However, the Trie algorithm scales much better when several scrambles are provided.  

## Usage

The Trie approach only allows for the longest word found to be returned, whereas the brute force approach allowed for refinement on the return value. For example: words above a certain length, top 10 longest words found, ... .  

```bash
cargo run <scramble>
```
