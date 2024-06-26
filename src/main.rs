use std::collections::HashSet;

fn is_pangram(sentence: &str) -> bool {
    let mut letters = HashSet::new();

    for ch in sentence.to_lowercase().chars() {
        if ch.is_ascii_alphabetic() {
            letters.insert(ch);
        }
    }

    letters.len() == 26
}

fn main() {
    let sentence = "The quick brown fox jumps over the lazy dog.";
    if is_pangram(sentence) {
        println!("The sentence is a pangram.");
    } else {
        println!("The sentence is not a pangram.");
    }
}

