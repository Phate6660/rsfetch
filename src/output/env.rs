use nixinfo::env;
use clap::ArgMatches;

#[cfg(feature = "pretty_output")]
use crate::shared_functions::table;
#[cfg(feature = "pretty_output")]
use prettytable::format::LinePosition::Intern;

#[cfg(feature = "pretty_output")]
pub fn output_editor(matches: &ArgMatches) {
    if matches.is_present("editor") {
        let mut table = table(' ', Intern, 7);
        table.add_row(row!["Editor", &env("EDITOR")]);
        table.printstd();
    }
}

#[cfg(feature = "pretty_output")]
pub fn output_shell(matches: &ArgMatches) {
    if matches.is_present("shell") {
        let mut table = table(' ', Intern, 8);
        table.add_row(row!["Shell", &env("SHELL")]);
        table.printstd();
    }
}

#[cfg(feature = "pretty_output")]
pub fn output_user(matches: &ArgMatches) {
    if matches.is_present("user") {
        let mut table = table(' ', Intern, 9);
        table.add_row(row!["User", &env("USER")]);
        table.printstd();
    }
}

#[cfg(not(feature = "pretty_output"))]
pub fn output_editor(matches: &ArgMatches) {
    if matches.is_present("editor") {
        println!("Editor:       {}", env("EDITOR"));
    }
}

#[cfg(not(feature = "pretty_output"))]
pub fn output_shell(matches: &ArgMatches) {
    if matches.is_present("shell") {
        println!("Shell:        {}", env("SHELL"));
    }
}

#[cfg(not(feature = "pretty_output"))]
pub fn output_user(matches: &ArgMatches) {
    if matches.is_present("user") {
        println!("User:         {}", env("USER"));
    }
}
