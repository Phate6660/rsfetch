use crate::info::uptime::uptime;
use clap::ArgMatches;

#[cfg(feature = "pretty_output")]
pub async fn async_uptime(matches: &ArgMatches<'_>) {
    if matches.is_present("uptime") {
        use crate::shared_functions::table;
        let mut table = table(7);
        table.add_row(row!["Uptime", &uptime()]);
        table.printstd();
    }
}

#[cfg(feature = "plain_output")]
pub async fn async_uptime(matches: &ArgMatches<'_>) {
    if matches.is_present("uptime") {
        println!("Uptime:       {}", uptime());
    }
}
