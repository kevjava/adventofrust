#![warn(missing_docs)]

//! This is my attempt at Day 3 of [Advent of Rust](https://adventofcode.com/2022/day/3). 
//! 
//! I'm apparently learning how to document crates too.

mod input_file;
mod rucksack;
mod elf_group;
mod priority;

use std::collections::HashSet;

use crate::input_file::InputFile;
use crate::rucksack::Rucksack;
use crate::elf_group::ElfGroup;

/// Makes rucksacks from the items.
fn get_rucksacks(lines: Vec<String>) -> Vec<Rucksack> {
    let mut rucksacks: Vec<Rucksack> = vec![];
    for line in &lines {
        rucksacks.push(Rucksack::new(line));
    }
    rucksacks
}

fn get_elf_groups(rucksacks: &Vec<Rucksack>) -> Vec<ElfGroup> {
    let mut i = 0;
    let mut elf_groups: Vec<ElfGroup> = vec![];
    loop {
        if i+3 <= rucksacks.len() {
            elf_groups.push(ElfGroup::new( (
                Rucksack::new( &rucksacks[i].get_items() ), // Can't copy strings.
                Rucksack::new( &rucksacks[i+1].get_items() ),
                Rucksack::new( &rucksacks[i+2].get_items() )
            ) ) );
        } else {
            break;
        }
        i += 3;
    } 
    elf_groups
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
    //     let common_chars = sack.get_common_chars();
    //     println!("Sack: {sack:?}. \n Common chars: {common_chars:?} \n Priority: {priority}\n");
    // }

    let total_priority: usize = rucksacks.iter()
        .map( |sack| sack.get_priority() )
        .sum();

    println!("Total priority: {total_priority}.");

    // Now, the second part.
    let elf_groups = get_elf_groups(&rucksacks);
    // for group in &elf_groups {
    //     let badge = group.get_badge();
    //     println!("Group: {group:?}. Badge: {badge:?}")
    // }

    let badges = elf_groups.iter()
        .map(|group| group.get_badge() )
        .collect::<Vec<HashSet<char>>>();

    let priority_sum: usize = badges.iter()
        .map(|badge| priority::get_priority_sum(badge))
        .collect::<Vec<usize>>()
        .iter()
        .sum();

    println!("Badge priority sum: {priority_sum}");
    Ok(())
}


