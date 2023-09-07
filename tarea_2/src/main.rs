use std::fs::File;
use std::io::{BufRead, BufReader};

const DIR_PATH: &str = "/Users/apocalix/Documents/GitHub/rust-software-dev/tarea_2/src/example.txt";

fn main() {
    let file = File::open(DIR_PATH).expect("File not found");
    let reader = BufReader::new(file);
    let mut valid_line_count = 0;

    for line in reader.lines(){
        let line = line.expect("Error reading line");
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() || 
            trimmed_line.starts_with("//") || 
            trimmed_line.len() < 3 || 
            trimmed_line.contains("return") 
        {
            continue;
        }
        valid_line_count += 1;
    }
    println!("Valid lines: {}", valid_line_count);
}
