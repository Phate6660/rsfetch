use nixinfo::distro;
use clap::ArgMatches;

#[cfg(feature = "pretty_output")]
pub fn output_distro(matches: &ArgMatches) {
    if matches.is_present("distro") {
        let mut table = crate::shared_functions::table(' ', prettytable::format::LinePosition::Intern, 7);
        table.add_row(row!["Distro", &distro().trim().replace("\"", "")]);
        table.printstd();
    }
}

#[cfg(feature = "plain_output")]
pub fn output_distro(matches: &ArgMatches) {
    if matches.is_present("distro") {
        println!("Distro:       {}", distro().trim().replace("\"", ""));
    }
}
