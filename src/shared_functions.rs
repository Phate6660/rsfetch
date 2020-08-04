use std::fs::File;
use std::io::{BufReader, Read};

pub fn vector(file: File, line: usize) -> Result<String, Box<dyn std::error::Error>> {
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let file_vector: Vec<&str> = contents.split('\n').collect();
    Ok(file_vector[line].to_string())
}
