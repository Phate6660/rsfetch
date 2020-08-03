use clap::{App, Arg};
mod info;
use info::distro::distro;
use log::error;

fn main() {
    let matches = App::new("rsfetch")
        .version("0.1.0")
        .about("\nAn info fetch tool written in Rust. Everything is off by default, enable what you want.")
        .arg(Arg::with_name("distro")
             .short("d")
             .long("distro")
             .help("Display the name of the distro."))
        .get_matches();
    if matches.is_present("distro") {
        println!("Distro: {}", distro());
    } else {
        error!("You must enable some options to use this. Please run `--help` to view them.");
    }
}
