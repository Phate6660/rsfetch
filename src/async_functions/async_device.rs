use crate::info::device::device;
use clap::ArgMatches;

#[cfg(feature = "pretty_output")]
pub async fn async_device(matches: &ArgMatches<'_>) {
    if matches.is_present("device") {
        use crate::shared_functions::table;
        let mut table = table(7);
        table.add_row(row!["Device", &device().trim()]);
        table.printstd();
    }
}

#[cfg(feature = "plain_output")]
pub async fn async_device(matches: &ArgMatches<'_>) {
    if matches.is_present("device") {
        println!("Device:       {}", device().trim());
    }
}
