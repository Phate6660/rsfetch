use clap::ArgMatches;
use crate::info::device::device;

pub async fn async_device(matches: &ArgMatches<'_>) {
    if matches.is_present("device") {
        println!("Device:    {}", device().trim());
    }
}
