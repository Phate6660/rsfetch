use crate::info::kernel::kernel;
use clap::ArgMatches;

pub async fn async_kernel(matches: &ArgMatches<'_>) {
    if matches.is_present("kernel") {
        println!("Kernel:       {}", kernel().trim());
    }
}
