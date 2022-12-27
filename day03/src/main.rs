#![warn(missing_docs)]

//! This is my attempt at Day 3 of [Advent of Rust](https://adventofcode.com/2022/day/3). 
//! 
//! I'm apparently learning how to document crates too.

mod input_file;
mod rucksack;

use crate::input_file::InputFile;
use crate::rucksack::Rucksack;

/// Makes rucksacks from the items.
fn get_rucksacks(lines: Vec<String>) -> Vec<Rucksack> {
    let mut rucksacks: Vec<Rucksack> = vec![];
    for line in &lines {
        rucksacks.push(Rucksack::new(line));
    }
    rucksacks
}

/// Main function for [Day 3](https://adventofcode.com/2022/day/3).
/// 
/// Each rucksack contains a list of "items", which are divided into two equal 
/// "compartments", front and back. We want to find a list of items that are 
/// common to both compartments of a rucksack, look up the priority for those 
/// items, and add them all up over all rucksacks. 
/// 
/// Input is in "input.txt".
fn main() -> Result<(), String> {
    let infile: InputFile = input_file::InputFile::new("input.txt");

    // if let Ok(lines) = infile.read_lines() {
    let lines = match infile.read_lines() {
        Ok(lines) => {
            lines
        }
        Err(message) => {
            return Err( format!("Error opening input file: {message}") );
        }
    };

    let rucksacks = get_rucksacks(lines);

    // for sack in &rucksacks {
    //     let priority = sack.get_priority();
    //     println!("Sack: {sack:?}. Priority: {priority}")
    // }

    let total_priority: usize = rucksacks.iter()
        .map( |sack| sack.get_priority() )
        .sum();

    println!("Total priority: {total_priority}.");
    Ok(())
}

