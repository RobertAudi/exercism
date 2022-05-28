// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut matched_words: HashMap<&str, i32> = HashMap::new();
    let mut note_words: HashMap<&str, i32> = HashMap::new();

    note.iter().for_each(|word| {
        *note_words.entry(word).or_insert(0) += 1;
    });

    magazine.iter().for_each(|word| {
        *matched_words.entry(word).or_insert(0) += 1;
    });

    note_words.iter().all(|(word, count)|{
        matched_words.get(word).unwrap_or(&0) >= count
    })
}
