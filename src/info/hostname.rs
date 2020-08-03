use log::error;
use std::fs::{metadata, read_to_string};

pub fn hostname() -> String {
    if metadata("/etc/hostname").is_ok() {
        read_to_string("/etc/hostname").unwrap()
    } else {
        error!("Could not obtain hostname.");
        "N/A".to_string()
    }
}
