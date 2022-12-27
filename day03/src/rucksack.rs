
use std::collections::HashSet;

use crate::priority;


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
        (items[0..((len/2))].to_string(), items[(len/2)..len].to_string())
    }

    /// Gets common characters present in both compartments.
    pub fn get_common_chars(&self) -> HashSet<char> {
        self.compartments.0.chars()
            .filter(|c| self.compartments.1.chars().any( |x| c == &x ))
            .collect::<HashSet<_>>()
    }

    pub fn get_priority(&self) -> usize {
        self.get_common_chars()
            .iter()
            .map(|c| priority::get_priority(c) )
            .sum()
    }

    pub(crate) fn get_items(&self) -> String {
        self._items.to_string()
    }

    pub(crate) fn contains_item(&self, c: &char) -> bool {
        self._items.contains(*c)
    }
}
