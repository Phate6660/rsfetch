use std::fs::{metadata, read_to_string};

pub fn hostname() -> String {
    if metadata("/etc/hostname").is_ok() {
        read_to_string("/etc/hostname").unwrap()
    } else {
        "N/A (could not read /etc/hostname)".to_string()
    }
}
