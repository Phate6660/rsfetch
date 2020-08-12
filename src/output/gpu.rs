use clap::ArgMatches;
use nixinfo::gpu;

#[cfg(feature = "pretty_output")]
pub fn output_gpu(matches: &ArgMatches) {
    if matches.is_present("gpu") {
        let mut table = crate::shared_functions::table('│', prettytable::format::LinePosition::Top, 10);
        table.add_row(row!["GPU", &gpu()]);
        table.printstd();
    }
}

#[cfg(not(feature = "pretty_output"))]
pub fn output_gpu(matches: &ArgMatches) {
    if matches.is_present("cpu") {
        println!("GPU:          {}", gpu());
    }
}
