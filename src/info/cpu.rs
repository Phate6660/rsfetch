pub fn cpu() -> String {
    if std::fs::metadata("/proc/cpuinfo").is_ok() {
        let file = std::fs::File::open("/proc/cpuinfo").unwrap();
        let line = crate::shared_functions::line(file, 4); // Expects model name to be on 5th line
        let line_vec: Vec<&str> = line.split(':').collect();
        line_vec[1].to_string()
    } else {
        "N/A (could not obtain cpu model)".to_string()
    }
}
