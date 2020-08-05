use crate::info::hostname::hostname;
use clap::ArgMatches;

pub async fn async_hostname(matches: &ArgMatches<'_>) {
    if matches.is_present("hostname") {
        println!("Hostname:     {}", hostname().trim());
    }
}
