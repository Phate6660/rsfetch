use clap::ArgMatches;
use crate::info::kernel::kernel;

pub async fn async_kernel(matches: &ArgMatches<'_>) {
    if matches.is_present("kernel") {
        println!("Kernel:    {}", kernel().trim());
    }
}
