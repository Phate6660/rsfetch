use crate::info::hostname::hostname;
use clap::ArgMatches;

#[cfg(feature = "pretty_output")]
pub async fn async_hostname(matches: &ArgMatches<'_>) {
    if matches.is_present("hostname") {
        use crate::shared_functions::table;
        use prettytable::format::LinePosition::Intern;
        let mut table = table(' ', Intern, 5);
        table.add_row(row!["Hostname", &hostname().trim()]);
        table.printstd();
    }
}

#[cfg(feature = "plain_output")]
pub async fn async_hostname(matches: &ArgMatches<'_>) {
    if matches.is_present("hostname") {
        println!("Hostname:     {}", hostname().trim());
    }
}
