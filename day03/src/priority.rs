use std::collections::HashSet;

    const PRIORITIES : &str = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    pub(crate) fn get_priority(c: &char) -> usize {
        PRIORITIES.chars().position(|x| x == *c).unwrap_or(0)
    }

    pub(crate) fn get_priority_sum(items: &HashSet<char>) -> usize {
        items
            .iter()
            .map(|c| get_priority(c))
            .collect::<Vec<usize>>()
            .iter()
            .sum()
    }