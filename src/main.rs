use clap::{App, Arg};

mod info;
use info::{distro::distro, env::env, kernel::kernel};

fn main() {
    let matches = App::new("rsfetch")
        .version("0.1.0")
        .author("Phate6660 <https://pages.codeberg.org/Phate6660>")
        .about("\nAn info fetch tool written in Rust. Everything is off by default, enable what you want.")
        .arg(Arg::with_name("distro")
             .short("d")
             .help("Display the name of the distro."))
        .arg(Arg::with_name("editor")
             .short("e")
             .help("Display the name of the user's editor. Must have the $EDITOR environmental variable set."))
        .arg(Arg::with_name("kernel")
             .short("k")
             .help("Display the name of the kernel."))
        .arg(Arg::with_name("shell")
             .short("s")
             .help("Display the name of the user's shell."))
        .arg(Arg::with_name("user")
             .short("u")
             .help("Display the name of the user."))
        .get_matches();
    if matches.is_present("distro") {
        println!("Distro: {}", distro().trim());
    }
    if matches.is_present("editor") {
        println!("Editor: {}", env("EDITOR".to_string()));
    }
    if matches.is_present("kernel") {
        println!("Kernel: {}", kernel().trim());
    }
    if matches.is_present("shell") {
        println!("Shell:  {}", env("SHELL".to_string()));
    }
    if matches.is_present("user") {
        println!("User:   {}", env("USER".to_string()));
    }
}
