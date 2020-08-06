use crate::info::dewm::environment;
use clap::ArgMatches;

#[cfg(feature = "pretty_output")]
pub async fn async_dewm(matches: &ArgMatches<'_>) {
    if matches.is_present("environment") {
        let mut table = crate::shared_functions::table(' ', prettytable::format::LinePosition::Intern, 2);
        table.add_row(row!["Environment", &environment().trim()]);
        table.printstd();
    }
}

#[cfg(feature = "plain_output")]
pub async fn async_dewm(matches: &ArgMatches<'_>) {
    if matches.is_present("environment") {
        println!("Environment:  {}", environment().trim());
    }
}
