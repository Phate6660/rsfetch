use std::fs::{metadata, File};
use std::io::{BufReader, Read};

fn line(file: File) -> Result<String, Box<dyn std::error::Error>> {
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let file_vector: Vec<&str> = contents.split('\n').collect();
    Ok(file_vector[4].to_string()) // Expects model name to be on 5th line
}

pub fn cpu() -> String {
    if metadata("/proc/cpuinfo").is_ok() {
        let file = File::open("/proc/cpuinfo").unwrap();
        let line = line(file).unwrap();
        let line_vec: Vec<&str> = line.split(':').collect();
        line_vec[1].to_string()
    } else {
        "N/A (could not obtain cpu model)".to_string()
    }
}
