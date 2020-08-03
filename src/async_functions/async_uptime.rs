use clap::ArgMatches;
use crate::info::uptime::uptime;

pub async fn async_uptime(matches: &ArgMatches<'_>) {
    if matches.is_present("uptime") {
        println!("Uptime:    {}", uptime());
    }
}
