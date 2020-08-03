use std::fs::read_to_string;

pub fn kernel() -> String {
    read_to_string("/proc/sys/kernel/osrelease").expect("Could not read file for kernel version.")
}
