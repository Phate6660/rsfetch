use crate::info::memory::memory;
use clap::ArgMatches;

pub async fn async_memory(matches: &ArgMatches<'_>) {
    if matches.is_present("memory") {
        println!("Memory:       {}", memory().trim());
    }
}
