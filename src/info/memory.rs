use crate::shared_functions::line;
use std::fs::{metadata, File};

pub fn memory() -> String {
    if metadata("/proc/meminfo").is_ok() {
        let file = File::open("/proc/meminfo").unwrap();
        let total_line = line(file, 0); // MemTotal should be on the first line
        let total_vec: Vec<&str> = total_line.split(':').collect();
        let total = total_vec[1].replace("kB", "");
        let total = total.trim().parse::<i64>().unwrap() / 1024;
        total.to_string() + " MB"
    } else {
        "N/A (could not read /proc/meminfo)".to_string()
    }
}
