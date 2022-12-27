
use std::collections::HashSet;

use crate::rucksack::Rucksack;

/// A group of three elves. Each has a rucksack.
#[derive(Debug)]
pub(crate) struct ElfGroup {
    _rucksacks: (Rucksack, Rucksack, Rucksack)
}

impl ElfGroup {

    pub fn new(rucksacks: (Rucksack, Rucksack, Rucksack)) -> ElfGroup {
        ElfGroup {
            _rucksacks: rucksacks,
        }
    }

    pub fn get_badge(&self) -> HashSet<char> {
        self._rucksacks.0.get_items().chars()
            .filter(|c| self._rucksacks.1.contains_item(c) && self._rucksacks.2.contains_item(c))
            .collect::<HashSet<char>>()
    }
}