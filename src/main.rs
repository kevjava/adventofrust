use core::str;
use std::fs;

fn get_groups(lines: Vec<&str>) -> Vec<Vec<&str>> {
    let mut groups : Vec<Vec<&str>> = Vec::new();
    let mut this_group: Vec<&str> = Vec::new();
    lines.into_iter().for_each(|x: &str| {
        if x.trim().is_empty() {
            // let l = this_group.len();
            groups.push(this_group.to_vec());
            // println!("Found {l} in that group.");
            this_group = Vec::new();
            // println!("\n New Group \n");
        } else {
            // println!("Found {x}");
            this_group.push(x);
        }
    });
    return groups;
}

/// Returns a sum from a vector of strings.
/// All strings will be parsed as i32 and summed up.
/// 
fn get_sum(group: Vec<&str>) -> i32  {
    let mut sum = 0;
    for line in group {
        let calories: i32 = line
            .trim()
            .parse()
            .expect("Couldn't parse a number from the line.");
        // println!("Adding {calories}");
        sum += calories;
    }
    // println!("Returning {sum}\n");
    return sum;
}

fn main() {
    let binding = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file.");
    let contents : Vec<&str> = binding
        .split("\n")
        .collect();

    let groups : Vec<Vec<&str>> = get_groups(contents);

    let mut biggest_sum = 0;
    for group in groups {
        let sum : i32 = get_sum(group);
        if sum > biggest_sum {
            biggest_sum = sum;
        }
    }
    println!("Biggest sum is {biggest_sum}");
}
