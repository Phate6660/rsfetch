use crate::info::distro::distro;
use clap::ArgMatches;

#[cfg(feature = "pretty_output")]
pub async fn async_distro(matches: &ArgMatches<'_>) {
    if matches.is_present("distro") {
        use crate::shared_functions::table;
        use prettytable::format::LinePosition::Intern;
        let mut table = table(' ', Intern, 7);
        table.add_row(row!["Distro", &distro().trim()]);
        table.printstd();
    }
}

#[cfg(feature = "plain_output")]
pub async fn async_distro(matches: &ArgMatches<'_>) {
    if matches.is_present("distro") {
        println!("Distro:       {}", distro().trim());
    }
}
