use std::io::{BufRead, BufReader, Lines, Result};
use std::{fs::File, path::Path};

pub fn read_numbers(filename: &str) -> Vec<i32> {
    let mut numbers = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                let number = l.parse::<i32>().unwrap();
                numbers.push(number);
            }
        }
    }
    return numbers;
}

pub fn read_larger_numbers(filename: &str) -> Vec<i64> {
    let mut numbers = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                let number = l.parse::<i64>().unwrap();
                numbers.push(number);
            }
        }
    }
    return numbers;
}

pub fn read_strings(filename: &str) -> Vec<String> {
    let mut strings = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                strings.push(l);
            }
        }
    }
    return strings;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
