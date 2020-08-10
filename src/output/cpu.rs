use clap::ArgMatches;
use nixinfo::{cpu, temp};

fn the_temp(matches: &ArgMatches) -> String {
    let unit = matches.value_of("temperature").unwrap();
    if unit == "C" {
        return temp().to_string() + "*C";
    } else if unit == "F" {
        let pre = temp().parse::<i64>().unwrap() * 9 / 5 + 32;
        return pre.to_string() + "*F";
    } else {
        format!("N/A ({} is not a supported unit)", unit)
    }
}

#[cfg(feature = "pretty_output")]
pub fn output_cpu(matches: &ArgMatches) {
    if matches.is_present("cpu") {
        let mut table = crate::shared_functions::table('│', prettytable::format::LinePosition::Top, 10);
        if matches.is_present("temperature") {
            let temp = the_temp(matches);
            let row = format!("{} [{}]", cpu().trim(), temp);
            table.add_row(row!["CPU", &row]);
        } else {
            table.add_row(row!["CPU", &cpu().trim()]);
        }
        table.printstd();
    }
}

#[cfg(feature = "plain_output")]
pub fn output_cpu(matches: &ArgMatches) {
    if matches.is_present("cpu") {
        if matches.is_present("temperature") {
            println!("CPU:          {} {}", cpu().trim(), the_temp());
        } else {
            println!("CPU:          {}", cpu().trim());
        }
    }
}
