use super::wc_helpers::DefWord;
use rayon::prelude::*;
use crate::DefChar;
use parlay::internal::group_by::histogram_by_key;
use std::hash::{Hash, Hasher, DefaultHasher};

pub fn wc<'a>(s: &'a mut Vec<DefChar>, res: &mut Vec<(String, i64)>) {
    let mut t = parlay::Timer::new("word counts");
    t.next("to_lower_case");
    //change all letters to lower case

    let s: Vec<DefChar> = s.par_iter().map(|&c| {
        match c {
            c if c >= 65 && c < 91 => c + 32,
            c if c >= 97 && c < 123 => c,
            _ => 0,
        }
    }).collect();
    t.next("split and remove empty words");
    // split into words in parallel
    let s: Vec<DefWord> = s.par_split(|&c| c == 0)
        .filter(|word| !word.is_empty())
        .map(|word| DefWord::new(word))
        .collect();
    t.next("histogram by key");
    //histrogram of the words
    let mut result = Vec::new();
    histogram_by_key::<DefWord, i64, _>(&s, word_hash, &mut result);
    *res = result.into_par_iter().map(|(k, v)| (k.to_string(), v)).collect();
    t.next("done");

}

fn word_hash<T: Hash>(t: T) -> usize {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish() as usize
}

