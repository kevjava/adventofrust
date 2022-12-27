#![warn(missing_docs)]

//! This is my attempt at Day 3 of [Advent of Rust](https://adventofcode.com/2022/day/3). 
//! 
//! I'm apparently learning how to document crates too.

mod input_file;

use input_file::InputFile;

/// Main function for [Day 3](https://adventofcode.com/2022/day/3).
/// 
/// Each rucksack contains a list of "items", which are divided into two equal 
/// "compartments", front and back. We want to find a list of items that are 
/// common to both compartments of a rucksack, look up the priority for those 
/// items, and add them all up over all rucksacks. 
/// 
/// Input is in "input.txt".
fn main() {
    let infile: InputFile = input_file::InputFile::new("input.txt");

    // if let Ok(lines) = infile.read_lines() {
    let lines = match (infile.read_lines()) {
        Ok(lines) => {
            lines
        }
        Err(message) => {
            panic!("Error opening input file: {message}");
        }
    };

    for line in &lines {
        println!("{line:?}");
    }
}
