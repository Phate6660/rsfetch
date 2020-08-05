use crate::info::packages::packages;
use clap::ArgMatches;

pub async fn async_packages(matches: &ArgMatches<'_>) {
    if matches.is_present("packages") {
        let manager = matches.value_of("packages").unwrap();
        println!("Packages:     {}", packages(manager).trim());
    }
}
