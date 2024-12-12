
use super::wc_helpers::DefWord;
use crate::DefChar;
use std::collections::HashMap;

pub fn wc<'a>(s: &'a mut Vec<DefChar>, res: &mut Vec<(String, i64)>) {
    let mut t = parlay::Timer::new("word counts");
    // copy to mutable vector and convert non alpha chars to spaces    
    t.next("to_lower_case");
    //change all letters to lower case
    t.next("copy");
    let s: Vec<DefChar> = s.iter().map(|&c| {
        match c {
            c if c >= 65 && c < 91 => c + 32,
            c if c >= 97 && c < 123 => c,
            _ => 0,
        }
    }).collect();
    t.next("split and remove empty words");
    // split into words
    let s: Vec<DefWord> = s.split(|&c| c == 0)
                           .filter(|word| !word.is_empty())
                           .map(|word| DefWord::new(word))
                           .collect();
    t.next("histogram by key");
    // count words
    let mut counts = HashMap::new();
    for w in &s {
        *counts.entry(w).or_insert(0) += 1;
    }
    t.next("count_sort");
    *res = counts.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
    t.next("done");
}