use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // Helper function to sort characters of a word in a case-insensitive manner
    fn sort_characters(s: &str) -> Vec<char> {
        let mut chars: Vec<char> = s.chars().flat_map(|c| c.to_lowercase()).collect();
        chars.sort_unstable();
        chars
    }

    // Sorted characters of the given word
    let sorted_word = sort_characters(word);

    // Collect anagrams
    possible_anagrams.iter()
        .filter(|&&candidate| {
            let candidate_sorted = sort_characters(candidate);
            candidate.to_lowercase() != word.to_lowercase() && candidate_sorted == sorted_word
        })
        .cloned()
        .collect()
}