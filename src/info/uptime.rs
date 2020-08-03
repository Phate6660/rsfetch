use std::fs::{metadata, read_to_string};

pub fn uptime() -> String {
    if metadata("/proc/uptime").is_ok() {
        let raw_uptime = read_to_string("/proc/uptime").unwrap();
        let uptime_vec: Vec<&str> = raw_uptime.split(".").collect();
        let uptime = uptime_vec[0].parse::<i64>().unwrap();
        if uptime > 86400 {
            let t = uptime / 60 / 60 / 24;
            return t.to_string() + &" days".to_string()
        } else if uptime > 3600 {
            let t = uptime / 60 / 60;
            return t.to_string() + &" hours".to_string()
        } else if uptime > 60 {
            let t = uptime / 60;
            return t.to_string() + &" minutes".to_string()
        } else {
            return "N/A (could not calculate time)".to_string()
        };
    } else {
        "N/A (could not obtain read /proc/uptime)".to_string()
    }
}
