use std::fs::{File, metadata};

fn get(file: File, x: usize) -> String {
    let line = crate::shared_functions::line(file, x);
    let line_vec: Vec<&str> = line.split(':').collect();
    line_vec[1].to_string()
}

pub fn cpu() -> String {
    if metadata("/proc/cpuinfo").is_ok() {
        let file = File::open("/proc/cpuinfo").unwrap();
        if metadata("/sys/firmware/devicetree/base/model").is_ok() {
            if std::fs::read_to_string("/sys/firmware/devicetree/base/model").unwrap().starts_with("Raspberry") {
                get(file, 1) // Line 2
            } else {
                get(file, 4) // Line 5
            }
        } else {
            get(file, 4) // Line 5
        }
    } else {
        "N/A (could not obtain cpu model)".to_string()
    }
}
