use crate::info::device::device;
use clap::ArgMatches;

pub async fn async_device(matches: &ArgMatches<'_>) {
    if matches.is_present("device") {
        println!("Device:       {}", device().trim());
    }
}
