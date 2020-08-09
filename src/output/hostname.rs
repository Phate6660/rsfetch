use crate::info::hostname::hostname;
use clap::ArgMatches;

#[cfg(feature = "pretty_output")]
pub fn output_hostname(matches: &ArgMatches) {
    if matches.is_present("hostname") {
        let mut table = crate::shared_functions::table(' ', prettytable::format::LinePosition::Intern, 5);
        table.add_row(row!["Hostname", &hostname().trim()]);
        table.printstd();
    }
}

#[cfg(feature = "plain_output")]
pub fn output_hostname(matches: &ArgMatches) {
    if matches.is_present("hostname") {
        println!("Hostname:     {}", hostname().trim());
    }
}
