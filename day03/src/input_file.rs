//! Reads input files for Advent of Code projects. 
//! Utility class.

use std::{fs};

#[derive(Debug)]
pub struct InputFile {
    path: String,
}

impl InputFile {
    /// Creates a new [InputFile] from a `str` path.
    pub fn new(path: &str) -> InputFile {
        InputFile {
            path: String::from(path)
        }
    }

    /// Splits the file's contents into lines and returns it as a 
    /// `Vec<String>`.
    pub fn read_lines(self: Self) -> Result<Vec<String>, String> {
        match fs::read_to_string(self.path) {
            Ok(contents) => {
                // FIXME: There has to be a more functional way to do this.
                let mut lines: Vec<String> = vec![];
                for line in contents.split("\n") {
                    lines.push(String::from(line));
                }
                Ok(lines.clone())
            }
            Err(error) => {
                Err(error.to_string())
            }
        }
    }
}