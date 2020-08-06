use crate::info::cpu::cpu;
use clap::ArgMatches;

#[cfg(feature = "pretty_output")]
pub async fn async_cpu(matches: &ArgMatches<'_>) {
    if matches.is_present("cpu") {
        use crate::shared_functions::table;
        use prettytable::format::LinePosition::Top;
        let mut table = table('│', Top, 10);
        table.add_row(row!["CPU", &cpu().trim()]);
        table.printstd();
    }
}

#[cfg(feature = "plain_output")]
pub async fn async_cpu(matches: &ArgMatches<'_>) {
    if matches.is_present("cpu") {
        println!("CPU:          {}", cpu().trim());
    }
}
