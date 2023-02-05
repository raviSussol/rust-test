use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};

pub fn read_file_as_buffer_string(file_path: &str) -> Result<String, std::io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let contents = reader
        .lines()
        .map(|line| line.unwrap_or("".to_string()))
        .collect();
    Ok(contents)
}

pub fn read_file_as_string(file_path: &str) -> Result<String, std::io::Error> {
    let contents = read_to_string(file_path)?;
    Ok(contents)
}
