use std::fs::{metadata, read_to_string};

pub fn kernel() -> String {
    if metadata("/proc/sys/kernel/osrelease").is_ok() {
        read_to_string("/proc/sys/kernel/osrelease").unwrap()
    } else {
        "N/A (could not obtain kernel version)".to_string()
    }
}
