use std::{fs::File, io::{BufRead, BufReader}};

pub struct WordNode {
    valid: bool,
    lower_letters: Vec<char>,
    lower_nodes: Vec<WordNode>,
}
impl WordNode {
    pub fn blank() -> Self {
        Self {
            valid: false,
            lower_letters: Vec::new(),
            lower_nodes: Vec::new(),
        }
    }
}

pub fn generate_tree(word_file: BufReader<File>) -> WordNode {
    let mut root = WordNode::blank();

    for line in word_file.lines() {
        let new_word = line.unwrap();

        let mut curr_node = &mut root;
        for c in new_word.chars() {
            if let Some(i) = curr_node.lower_letters.iter().position(|&r| r == c) {
                curr_node = &mut curr_node.lower_nodes[i];
                continue;
            }

            curr_node.lower_letters.push(c);
            curr_node.lower_nodes.push(WordNode::blank());

            let last_index = curr_node.lower_nodes.len() - 1;
            curr_node = &mut curr_node.lower_nodes[last_index];
        }
        curr_node.valid = true;
    }
    root
}
pub fn find_word(tree: &WordNode, letters: &String) -> String {
    let letters: Vec<char> = letters.chars().collect();
    search_word(tree, &letters).1.iter().rev().collect()
}

fn search_word(tree: &WordNode, letters: &Vec<char>) -> (bool, Vec<char>) {
    let mut combo = Vec::new();

    for letter in letters {
        let position = tree.lower_letters.iter().position(|r| r == letter);
        let index = match position {
            Some(i) => i,
            None => continue,
        };

        let mut temp = letters.clone();
        temp.remove(letters.iter().position(|r| r == letter).unwrap());

        let (valid_word, mut lower_word) = search_word(&tree.lower_nodes[index], &temp);
        if lower_word.len() >= combo.len() && valid_word {
            lower_word.push(*letter);
            combo = lower_word;
            
        }     
    }

    return (combo.len() != 0 || tree.valid, combo);
}