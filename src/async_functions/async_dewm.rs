use crate::info::dewm::environment;
use clap::ArgMatches;

#[cfg(feature = "pretty_output")]
pub async fn async_dewm(matches: &ArgMatches<'_>) {
    if matches.is_present("environment") {
        use crate::shared_functions::table;
        use prettytable::format::LinePosition::Intern;
        let mut table = table(' ', Intern, 2);
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
