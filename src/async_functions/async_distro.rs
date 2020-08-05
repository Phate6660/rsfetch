use crate::info::distro::distro;
use clap::ArgMatches;

pub async fn async_distro(matches: &ArgMatches<'_>) {
    if matches.is_present("distro") {
        println!("Distro:       {}", distro().trim());
    }
}
