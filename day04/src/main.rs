mod input_file;
mod assignment_pair;

use crate::input_file::InputFile;
use crate::assignment_pair::AssignmentPair;

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

    let assignment_pairs = lines.iter()
        .map(|line| AssignmentPair::new(line))
        .collect::<Vec<AssignmentPair>>();

    // for pair in &assignment_pairs {
    //     println!("Pair: {first:?},{second:?}. Contains: {contains} Overlaps: {overlaps}.", 
    //         first=pair.first(), second=pair.second(), 
    //         contains=pair.contains(), overlaps=pair.overlaps());
    // }

    let containing_assignment_pairs_count = assignment_pairs
        .iter()
        .filter( |pair| pair.contains() )
        .count();
    println!("Containing assignment pairs: {containing_assignment_pairs_count}");
    
    let overlapping_assignment_pairs_count = assignment_pairs
        .iter()
        .filter( |pair| pair.overlaps() )
        .count();
    println!("Overlapping assignment pairs: {overlapping_assignment_pairs_count}");
    
    Ok(())
}
