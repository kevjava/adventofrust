use core::str;
use std::fs;

fn main() {
    println!("Hello, world!");

    let binding = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file.");

    let contents : Vec<&str> = binding
        .split("\n")
        .collect();

    contents.iter().for_each(|s| {
        println!("Line: {s}");
    });
}
