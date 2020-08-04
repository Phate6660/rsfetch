use crate::info::cpu::cpu;
use clap::ArgMatches;

pub async fn async_cpu(matches: &ArgMatches<'_>) {
    if matches.is_present("cpu") {
        println!("CPU:       {}", cpu().trim());
    }
}
