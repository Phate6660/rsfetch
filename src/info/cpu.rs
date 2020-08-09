use std::fs::{File, metadata, read_to_string};

fn get(file: File, x: usize) -> String {
    let line = crate::shared_functions::line(file, x);
    let line_vec: Vec<&str> = line.split(':').collect();
    line_vec[1].to_string()
}

fn format(info: String) -> String {
    info.replace("(TM)", "")
        .replace("(R)", "")
        .replace("     ", " ")
}

pub fn cpu(matches: &clap::ArgMatches<'_>) -> String {
    if metadata("/proc/cpuinfo").is_ok() {
        let file = File::open("/proc/cpuinfo").unwrap();
        let model = if metadata("/sys/firmware/devicetree/base/model").is_ok() {
            if read_to_string("/sys/firmware/devicetree/base/model").unwrap().starts_with("Raspberry") {
                let info = get(file, 1); // Line 2
                format(info)
            } else {
                let info = get(file, 4); // Line 5
                format(info)
            }
        } else {
            let info = get(file, 4); // Line 5
            format(info)
        };
        let temp = if matches.is_present("temperature") {
            if metadata("/sys/class/thermal/thermal_zone0/temp").is_ok() {
                {
                    let file = File::open("/sys/class/thermal/thermal_zone0/temp").unwrap();
                    let raw_temp = crate::shared_functions::read(file).unwrap().trim().parse::<i64>().unwrap();
                    let temp_pre = raw_temp / 1000;
                    let unit = matches.value_of("temperature").unwrap();
                    if unit == "C" {
                        format!("[{}*C]", temp_pre)
                    } else if unit == "F" {
                        let temp_pre = temp_pre * 9 / 5 + 32;
                        format!("[{}*F]", temp_pre)
                    } else {
                        "N/A (invalid temperature unit)".to_string()
                    }
                }
            } else {
                "N/A (could not read /sys/class/thermal/thermal_zone0/temp)".to_string()
            }
        } else {
            "".to_string()
        };
        format!("{} {}", model, temp)
    } else {
        "N/A (could not read /proc/cpuinfo)".to_string()
    }
}
