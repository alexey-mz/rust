use std::collections::HashSet;

fn compare_words(a: &str, b: &str) -> bool {
    let mut word1 = a.chars().collect::<Vec<char>>();
    word1.sort();
    let mut word2 = b.chars().collect::<Vec<char>>();
    word2.sort();
    word1 == word2
}

fn main() {

    let word = "good";
    let word2 = "dog";
    println!("{:?}", compare_words(word, word2));
}
