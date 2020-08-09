use crate::info::kernel::kernel;
use clap::ArgMatches;

#[cfg(feature = "pretty_output")]
pub fn output_kernel(matches: &ArgMatches) {
    if matches.is_present("kernel") {
        let mut table = crate::shared_functions::table(' ', prettytable::format::LinePosition::Intern, 7);
        table.add_row(row!["Kernel", &kernel().trim()]);
        table.printstd();
    }
}

#[cfg(feature = "plain_output")]
pub fn output_kernel(matches: &ArgMatches) {
    if matches.is_present("kernel") {
        println!("Kernel:       {}", kernel().trim());
    }
}
