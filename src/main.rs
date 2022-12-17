use core::str;
use std::{fs, io::Error};

/// Returns an expect-able file result given a filename.
fn open_file(filename: &str) -> Result<String, Error> {
    return fs::read_to_string(filename);
}

/// Takes in a string, splits it by newlines, returns a Vec with the lines.
fn split_lines(file_contents: &str) -> Vec<&str> {
    return file_contents
        .split("\n")
        .collect();
}

/// Returns groups of lines, separated by blank lines.
/// Splits one big Vec into a Vec of Vecs, containing the groups.
fn get_groups(lines: Vec<&str>) -> Vec<Vec<&str>> {
    let mut groups : Vec<Vec<&str>> = Vec::new();
    let mut this_group: Vec<&str> = Vec::new();
    lines.into_iter().for_each(|x: &str| {
        if x.trim().is_empty() {
            groups.push(this_group.to_vec());
            this_group = Vec::new();
        } else {
            this_group.push(x);
        }
    });
    return groups;
}

/// Returns a sum from a vector of strings.
/// All strings will be parsed as i32 and summed up.
fn get_sum(group: Vec<&str>) -> i32  {
    let mut sum = 0;
    for line in group {
        let calories: i32 = line
            .trim()
            .parse()
            .expect("Couldn't parse a number from the line.");
        sum += calories;
    }
    return sum;
}

/// Day 1 of Advent of Code 2022.
fn main() {
    let input_file = open_file("input.txt")
        .expect("Couldn't open input file");
    let contents = split_lines(&input_file);

    let groups : Vec<Vec<&str>> = get_groups(contents);

    let mut list_of_sums: Vec<i32> = Vec::new();
    for group in groups {
        let sum : i32 = get_sum(group);
        list_of_sums.push(sum);
    }
    list_of_sums.sort_by( |a,b| { b.cmp(a) }); // Reverse sort.

    // for sum in &list_of_sums { print!("{sum} ")}
    let biggest_sum: i32 = list_of_sums[0];
    let top_3_sum: i32 = list_of_sums[0..3].to_vec().iter().sum();

    println!("Biggest sum: {biggest_sum}");
    println!("Sum of top three: {top_3_sum}");
}

