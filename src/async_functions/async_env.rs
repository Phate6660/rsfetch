use clap::ArgMatches;
use crate::info::env::env;

pub async fn async_editor(matches: &ArgMatches<'_>) {
    if matches.is_present("editor") {
        println!("Editor:    {}", env("EDITOR".to_string()));
    }
}

pub async fn async_shell(matches: &ArgMatches<'_>) {
    if matches.is_present("shell") {
        println!("Shell:     {}", env("SHELL".to_string()));
    }
}

pub async fn async_user(matches: &ArgMatches<'_>) {
    if matches.is_present("user") {
        println!("User:      {}", env("USER".to_string()));
    }
}
