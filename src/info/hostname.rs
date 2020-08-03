use log::error;
use std::fs::{metadata, read_to_string};

pub fn hostname() -> String {
    if metadata("/etc/hostname").is_ok() {
        read_to_string("/etc/hostname").expect("Could not read file for hostname.")
    } else {
        error!("Could not obtain hostname.");
        "N/A".to_string()
    }
}
