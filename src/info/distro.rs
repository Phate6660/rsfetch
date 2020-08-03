use log::error;
use regex::Regex;
use std::fs::{metadata, File};
use std::io::{BufReader, Read};

fn line(file: File) -> Result<String, Box<dyn std::error::Error>> {
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let file_vector: Vec<&str> = contents.split("\n").collect();
    let r = Regex::new("^NAME")?;
    let line: String = file_vector.into_iter().filter(|s| r.is_match(s)).collect();
    Ok(line)
}

pub fn distro() -> String {
    if metadata("/bedrock/etc/os-release").is_ok() {
        let file = File::open("/bedrock/etc/os-release").expect("Could not read /bedrock/etc/os-release.");
        let line: String = line(file).unwrap();
        let distro_vec: Vec<&str> = line.split("=").collect();
        String::from(distro_vec[1])
    } else if metadata("/etc/os-release").is_ok() {
        let file = File::open("/etc/os-release").expect("Could not read /etc/os-release.");
        let line: String = line(file).unwrap();
        let distro_vec: Vec<&str> = line.split("=").collect();
        String::from(distro_vec[1])
    } else if metadata("/usr/lib/os-release").is_ok() {
        let file = File::open("/usr/lib/os-release").expect("Could not read /usr/lib/os-release.");
        let line: String = line(file).unwrap();
        let distro_vec: Vec<&str> = line.split("=").collect();
        String::from(distro_vec[1])
    } else {
        error!("No suitable file was found. Please file a bug! Your os-release may just be in an odd location.");
        "N/A".to_string()
    }
}
