mod input_file;
mod crate_stacks;
mod instruction_set;

use std::cmp;
use std::collections::VecDeque;

use crate_stacks::Crate;
use crate_stacks::CrateStacks;
use input_file::InputFile;
use instruction_set::Instruction;
use instruction_set::InstructionSet;
use regex::Regex;

fn get_initial_crate_stacks(lines: &Vec<String>) -> Vec<VecDeque<Crate>> {
    let mut crate_lines = vec!();
    let mut max_len = 0;
    for line in lines {
        if line.trim().len() == 0 {
            break;
        }
        crate_lines.push(line);
        max_len = cmp::max(max_len, line.len());
    }

    let mut crate_stacks = vec!();
    for _ in 0..((max_len+1)/4) {
        crate_stacks.push(VecDeque::new());
    }

    let valid_crates = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let n_stacks = (max_len+1)/4;
    for line in crate_lines {
        for i in 1..=n_stacks {
            let pos = (i*4) - 3;
            let label = line.chars().nth(pos).unwrap_or(' ');
            if valid_crates.contains(label) {
                // println!("Found crate {label}");
                crate_stacks[i-1].push_front(Crate::new(label));
            }
        }
    }

    crate_stacks
}

fn get_instructions(lines: &Vec<String>) -> Vec<instruction_set::Instruction> {
    let mut instructions = vec!();
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in lines {
        for cap in re.captures_iter(&line) {
            instructions.push( Instruction {
                move_n: cap[1].parse::<usize>().unwrap_or(0),
                from: cap[2].parse::<usize>().unwrap_or(0),
                to: cap[3].parse::<usize>().unwrap_or(0)
            });
        }
    }

    instructions
}

fn main() -> Result<(), String> {
    let infile = InputFile::new("input.txt");
    let lines = match infile.read_lines() {
        Ok(lines) => { lines }
        Err(message) => { return Err( format!("Error opening input file: {message}") ); }
    };

    let mut crates = CrateStacks::new( get_initial_crate_stacks(&lines) );
    // println!("{crates:?}");

    let instruction_set = InstructionSet::new( get_instructions(&lines) );
    // println!("{instruction_set:?}");

    for instruction in instruction_set.instructions {
        // println!("{instruction:?}");
        crates.perform_instruction(&instruction);
        // println!("{crates:?}");
    }
    
    let top = crates.get_top();
    println!("CrateMover 9000: {top:?}");

    // println!("\n\n\n");

    let mut crates = CrateStacks::new( get_initial_crate_stacks(&lines) );
    // println!("Initial crates: {crates:?}");
    let instruction_set = InstructionSet::new( get_instructions(&lines));
    for instruction in instruction_set.instructions {
        // println!("{instruction:?}");
        crates.perform_instruction_multiple(&instruction);
        // println!("{crates:?}");
    }

    let top = crates.get_top();
    println!("CrateMover 9001: {top:?}");
    Ok(())
}

