use crate::info::dewm::environment;
use clap::ArgMatches;

pub async fn async_dewm(matches: &ArgMatches<'_>) {
    if matches.is_present("environment") {
        println!("Environment:  {}", environment().trim());
    }
}
