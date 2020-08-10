use nixinfo::device;
use clap::ArgMatches;

#[cfg(feature = "pretty_output")]
pub fn output_device(matches: &ArgMatches) {
    if matches.is_present("device") {
        let mut table = crate::shared_functions::table(' ', prettytable::format::LinePosition::Intern, 7);
        table.add_row(row!["Device", &device().trim()]);
        table.printstd();
    }
}

#[cfg(not(feature = "pretty_output"))]
pub fn output_device(matches: &ArgMatches) {
    if matches.is_present("device") {
        println!("Device:       {}", device().trim());
    }
}
