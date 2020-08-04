use crate::shared_functions::vector;
use std::fs::{metadata, File};

pub fn cpu() -> String {
    if metadata("/proc/cpuinfo").is_ok() {
        let file = File::open("/proc/cpuinfo").unwrap();
        let line = vector(file, 4).unwrap(); // Expects model name to be on 5th line
        let line_vec: Vec<&str> = line.split(':').collect();
        line_vec[1].to_string()
    } else {
        "N/A (could not obtain cpu model)".to_string()
    }
}
