use clap::{App, Arg, ArgMatches};
use futures::executor::block_on;

mod async_functions;
use async_functions::{
    async_device::async_device,
    async_distro::async_distro,
    async_env::{async_editor, async_shell, async_user},
    async_hostname::async_hostname,
    async_kernel::async_kernel,
    async_packages::async_packages,
    async_uptime::async_uptime,
};
mod info;

async fn async_main(matches: ArgMatches<'_>) {
    let f1 = async_device(&matches);
    let f2 = async_distro(&matches);
    let f3 = async_editor(&matches);
    let f4 = async_shell(&matches);
    let f5 = async_user(&matches);
    let f6 = async_hostname(&matches);
    let f7 = async_kernel(&matches);
    let f8 = async_packages(&matches);
    let f9 = async_uptime(&matches);
    futures::join!(f1, f2, f3, f4, f5, f6, f7, f8, f9);
}

fn main() {
    let matches = App::new("rsfetch")
        .version("0.1.0")
        .author("Phate6660 <https://pages.codeberg.org/Phate6660>")
        .about("\nAn info fetch tool written in Rust. Everything is off by default, enable what you want.")
        .arg(Arg::with_name("device")
             .short("D")
             .help("Display the name of the device."))
        .arg(Arg::with_name("distro")
             .short("d")
             .help("Display the name of the distro."))
        .arg(Arg::with_name("editor")
             .short("e")
             .help("Display the name of the user's editor. Must have the $EDITOR environmental variable set."))
        .arg(Arg::with_name("hostname")
             .short("h")
             .help("Display the hostname of the device."))
        .arg(Arg::with_name("kernel")
             .short("k")
             .help("Display the name of the kernel."))
        .arg(Arg::with_name("packages")
             .short("p")
             .long("packages")
             .value_name("PKG MNGR")
             .help("Turn total package count on.")
             .takes_value(true))
        .arg(Arg::with_name("shell")
             .short("s")
             .help("Display the name of the user's shell."))
        .arg(Arg::with_name("uptime")
             .short("u")
             .help("Display the uptime."))
        .arg(Arg::with_name("user")
             .short("U")
             .help("Display the name of the user."))
        .get_matches();
    block_on(async_main(matches));
}
