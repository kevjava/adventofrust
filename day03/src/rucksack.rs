use std::{collections::HashSet};




const PRIORITIES : &str = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[derive(Debug)]
pub struct Rucksack {
    _items: String,
    compartments: (String, String),
}

impl Rucksack {

    pub fn new(items: &String) -> Rucksack {
        let compartments = Rucksack::split_items(&items);
    
        Rucksack {
            _items: items.to_string(),
            compartments
        }
    }

    fn split_items(items: &String) -> (String, String) {
        let len = items.len();
        (items[0..((len/2)-1)].to_string(), items[(len/2)..len].to_string())
    }

    /// Gets common characters present in both compartments.
    fn get_common_chars(&self) -> HashSet<char> {
        self.compartments.0.chars()
            .filter(|c| self.compartments.1.chars().any( |x| c == &x ))
            .collect::<HashSet<_>>()
    }

    pub fn get_priority(&self) -> usize {
        self.get_common_chars()
            .iter()
            .map(|c| PRIORITIES.chars().position(|x| x == *c).unwrap_or(0) )
            .sum()
    }
}