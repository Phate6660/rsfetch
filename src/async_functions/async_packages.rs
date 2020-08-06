use crate::info::packages::packages;
use clap::ArgMatches;

#[cfg(feature = "pretty_output")]
pub async fn async_packages(matches: &ArgMatches<'_>) {
    if matches.is_present("packages") {
        use crate::shared_functions::table;
        use prettytable::format::LinePosition::Intern;
        let manager = matches.value_of("packages").unwrap();
        let mut table = table(' ', Intern, 5);
        table.add_row(row!["Packages", &packages(manager).trim()]);
        table.printstd();
    }
}

#[cfg(feature = "plain_output")]
pub async fn async_packages(matches: &ArgMatches<'_>) {
    if matches.is_present("packages") {
        let manager = matches.value_of("packages").unwrap();
        println!("Packages:     {}", packages(manager).trim());
    }
}
