use std::collections::HashSet;

mod input_file;

fn main() -> Result<(), String> {
    let infile = input_file::InputFile::new("input.txt");
    
    let lines = match infile.read_lines() {
        Ok(lines) => { lines }
        Err(message) => { return Err(format!("Error opening input file: {message}")) }
    };

    part1(&lines);
    part2(&lines);
    Ok(())
}

fn part1(lines: &Vec<String>) {
    for line in lines {
        let mut marker = 4;
        loop {
            if marker > line.len() { 
                println!("Line: {line}: reached the end.");
                break; 
            }
            let chars = line[marker-4..marker]
                .chars()
                .collect::<HashSet<char>>();
            if chars.len() == 4 {
                println!("Part 1: Marker={marker}");
                break;
            }
            marker += 1;
        }
    }
}

fn part2(lines: &Vec<String>) {
    for line in lines {
        let mut marker = 14;
        loop {
            if marker > line.len() { 
                println!("Line: {line}: reached the end.");
                break; 
            }
            let chars = line[marker-14..marker]
                .chars()
                .collect::<HashSet<char>>();
            if chars.len() == 14 {
                println!("Part 2: Marker={marker}");
                break;
            }
            marker += 1;
        }
    }
}
