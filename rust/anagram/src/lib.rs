use std::collections::{HashSet, HashMap};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {

    let word_lower = word.to_lowercase();
    let mut char_map = HashMap::new();
    word_lower
        .chars()
        .for_each(|c| *char_map.entry(c).or_insert(0) += 1);
    let check_char = |s: &str| -> bool {
        let mut char_map = char_map.clone();
        s.chars().any(|c| {
            let v = char_map.entry(c).or_insert(0);
            *v -= 1;
            *v < 0
        })
    };
    possible_anagrams
        .iter()
        .filter(|x| {
            x.len() == word.len() && {
                let x_lower = x.to_lowercase();
                x_lower != word_lower && !check_char(&x_lower)
            }
        })
        .cloned()
        .collect()
}
