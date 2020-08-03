use clap::ArgMatches;
use crate::info::distro::distro;

pub async fn async_distro(matches: &ArgMatches<'_>) {
    if matches.is_present("distro") {
        println!("Distro:    {}", distro().trim());
    }
}
