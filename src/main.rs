use clap::{App, Arg};
use log::error;

mod info;
use info::{distro::distro, kernel::kernel};

fn main() {
    let matches = App::new("rsfetch")
        .version("0.1.0")
        .about("\nAn info fetch tool written in Rust. Everything is off by default, enable what you want.")
        .arg(Arg::with_name("distro")
             .short("d")
             .help("Display the name of the distro."))
        .arg(Arg::with_name("kernel")
             .short("k")
             .help("Display the name of the kernel."))
        .get_matches();
    if matches.is_present("distro") {
        println!("Distro: {}", distro().trim());
    }
    if matches.is_present("kernel") {
        println!("Kernel: {}", kernel().trim());
    }
    if matches.is_present("") {
        error!("You must enable some options to use this. Please run `--help` to view them.");
    }
}
