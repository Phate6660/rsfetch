use std::fs::read_to_string;

pub fn hostname() -> String {
    read_to_string("/etc/hostname").expect("Could not read file for hostname.")
}
