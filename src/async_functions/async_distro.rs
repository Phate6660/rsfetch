use crate::info::distro::distro;
use clap::ArgMatches;

#[cfg(feature = "pretty_output")]
pub async fn async_distro(matches: &ArgMatches<'_>) {
    if matches.is_present("distro") {
        let mut table = crate::shared_functions::table(' ', prettytable::format::LinePosition::Intern, 7);
        table.add_row(row!["Distro", &distro().trim().replace("\"", "")]);
        table.printstd();
    }
}

#[cfg(feature = "plain_output")]
pub async fn async_distro(matches: &ArgMatches<'_>) {
    if matches.is_present("distro") {
        println!("Distro:       {}", distro().trim().replace("\"", ""));
    }
}
