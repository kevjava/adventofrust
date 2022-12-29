mod input_file;
mod crate_stacks;

use std::cmp;
use std::collections::VecDeque;

use crate_stacks::Crate;
use crate_stacks::CrateStacks;
use input_file::InputFile;

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
                println!("Found crate {label}");
                crate_stacks[i-1].push_front(Crate::new(label));
            }
        }
    }

    crate_stacks
}

fn main() -> Result<(), String> {
    let infile = InputFile::new("test.txt");
    let lines = match infile.read_lines() {
        Ok(lines) => { lines }
        Err(message) => { return Err( format!("Error opening input file: {message}") ); }
    };

    let crates = CrateStacks::new( get_initial_crate_stacks(&lines) );

    println!("{crates:?}");

    Ok(())
}
