use std::collections::HashSet;

fn compare_words(a: &str, b: &str) -> bool {
    let mut word1 = a.to_lowercase().chars().collect::<Vec<char>>();
    word1.sort();
    let mut word2 = b.to_lowercase().chars().collect::<Vec<char>>();
    word2.sort();
    word1 == word2
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams.iter().fold(HashSet::new(), |mut acc, &word_from_iter| {
        if word_from_iter.to_lowercase() == word.to_lowercase() {
            ()
        }
        else {
            if compare_words(word, word_from_iter) {
                acc.insert(word_from_iter);
            }
        }
        acc
    })
}
