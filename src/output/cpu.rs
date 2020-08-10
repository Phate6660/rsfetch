use nixinfo::cpu;
use clap::ArgMatches;

#[cfg(feature = "pretty_output")]
pub fn output_cpu(matches: &ArgMatches) {
    if matches.is_present("cpu") {
        let mut table = crate::shared_functions::table('│', prettytable::format::LinePosition::Top, 10);
        table.add_row(row!["CPU", &cpu().trim()]);
        table.printstd();
    }
}

#[cfg(feature = "plain_output")]
pub fn output_cpu(matches: &ArgMatches) {
    if matches.is_present("cpu") {
        println!("CPU:          {}", cpu().trim());
    }
}
