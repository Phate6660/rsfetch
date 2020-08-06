use crate::info::memory::memory;
use clap::ArgMatches;

#[cfg(feature = "pretty_output")]
pub async fn async_memory(matches: &ArgMatches<'_>) {
    if matches.is_present("memory") {
        let mut table = crate::shared_functions::table(' ', prettytable::format::LinePosition::Intern, 7);
        table.add_row(row!["Memory", &memory().trim()]);
        table.printstd();
    }
}

#[cfg(feature = "plain_output")]
pub async fn async_memory(matches: &ArgMatches<'_>) {
    if matches.is_present("memory") {
        println!("Memory:       {}", memory().trim());
    }
}
