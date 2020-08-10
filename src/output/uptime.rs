use nixinfo::uptime;
use clap::ArgMatches;

#[cfg(feature = "pretty_output")]
pub fn output_uptime(matches: &ArgMatches) {
    if matches.is_present("uptime") {
        let mut table = crate::shared_functions::table(' ', prettytable::format::LinePosition::Intern, 7);
        table.add_row(row!["Uptime", &uptime()]);
        table.printstd();
    }
}

#[cfg(feature = "plain_output")]
pub fn output_uptime(matches: &ArgMatches) {
    if matches.is_present("uptime") {
        println!("Uptime:       {}", uptime());
    }
}
