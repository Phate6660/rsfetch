use nixinfo::memory;
use clap::ArgMatches;

#[cfg(feature = "pretty_output")]
pub fn output_memory(matches: &ArgMatches) {
    if matches.is_present("memory") {
        let mut table = crate::shared_functions::table(' ', prettytable::format::LinePosition::Intern, 7);
        table.add_row(row!["Memory", &memory().trim()]);
        table.printstd();
    }
}

#[cfg(not(feature = "pretty_output"))]
pub fn output_memory(matches: &ArgMatches) {
    if matches.is_present("memory") {
        println!("Memory:       {}", memory().trim());
    }
}
