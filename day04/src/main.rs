mod input_file;

use crate::input_file::InputFile;

fn main() -> Result<(), String> {
    let infile: InputFile = input_file::InputFile::new("test.txt");

    // if let Ok(lines) = infile.read_lines() {
    let lines = match infile.read_lines() {
        Ok(lines) => {
            lines
        }
        Err(message) => {
            return Err( format!("Error opening input file: {message}") );
        }
    };

    

    Ok(())
}
