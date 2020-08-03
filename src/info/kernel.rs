use log::error;
use std::fs::{metadata, read_to_string};

pub fn kernel() -> String {
    if metadata("/proc/sys/kernel/osrelease").is_ok() {
        read_to_string("/proc/sys/kernel/osrelease")
            .expect("Could not read file for kernel version.")
    } else {
        error!("Could not obtain kernel version.");
        "N/A".to_string()
    }
}
