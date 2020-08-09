#[cfg(feature = "pretty_output")]
#[macro_use] extern crate prettytable;

use clap::Arg;

mod output;
mod info;
mod shared_functions;

fn main() {
    let matches = clap::App::new("rsfetch")
        .version("0.1.0")
        .author("Phate6660 <https://pages.codeberg.org/Phate6660>")
        .about("\nAn info fetch tool written in Rust. Everything is off by default, enable what you want.")
        .arg(Arg::with_name("cpu")
             .short("c")
             .help("Display the model of the CPU."))
        .arg(Arg::with_name("device")
             .short("D")
             .help("Display the name of the device."))
        .arg(Arg::with_name("distro")
             .short("d")
             .help("Display the name of the distro."))
        .arg(Arg::with_name("editor")
             .short("E")
             .help("Display the name of the user's editor. Must have the $EDITOR environmental variable set."))
        .arg(Arg::with_name("environment")
             .short("e")
             .help("Display the user's environment. First checks for a DE, before resorting to parsing your $HOME/.xinitrc for your WM."))
        .arg(Arg::with_name("hostname")
             .short("h")
             .help("Display the hostname of the device."))
        .arg(Arg::with_name("kernel")
             .short("k")
             .help("Display the name of the kernel."))
        .arg(Arg::with_name("memory")
             .short("m")
             .help("Display free/total memory."))
        .arg(Arg::with_name("music")
             .short("M")
             .help("Display currently playing music. Only mpd is supported. Must be built with the music feature."))
        .arg(Arg::with_name("packages")
             .short("p")
             .long("packages")
             .value_name("manager")
             .help("Display package count.")
             .takes_value(true))
        .arg(Arg::with_name("shell")
             .short("s")
             .help("Display the name of the user's shell."))
        .arg(Arg::with_name("temperature")
             .short("T")
             .long("temperature")
             .value_name("C/F")
             .help("Display CPU temp for Raspberry Pi, must have CPU field enabled.")
             .takes_value(true))
        .arg(Arg::with_name("terminal")
             .short("t")
             .help("Display the name of the user's terminal."))
        .arg(Arg::with_name("uptime")
             .short("u")
             .help("Display the uptime."))
        .arg(Arg::with_name("user")
             .short("U")
             .help("Display the name of the user."))
        .get_matches();
    output::main::main(matches);
}
