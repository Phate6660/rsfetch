use crate::info::env::env;
use clap::ArgMatches;

#[cfg(feature = "pretty_output")]
pub async fn async_editor(matches: &ArgMatches<'_>) {
    if matches.is_present("editor") {
        use crate::shared_functions::table;
        let mut table = table(7);
        table.add_row(row!["Editor", &env("EDITOR".to_string())]);
        table.printstd();
    }
}

#[cfg(feature = "pretty_output")]
pub async fn async_shell(matches: &ArgMatches<'_>) {
    if matches.is_present("shell") {
        use crate::shared_functions::table;
        let mut table = table(8);
        table.add_row(row!["Shell", &env("SHELL".to_string())]);
        table.printstd();
    }
}

#[cfg(feature = "pretty_output")]
pub async fn async_user(matches: &ArgMatches<'_>) {
    if matches.is_present("user") {
        use crate::shared_functions::table;
        let mut table = table(9);
        table.add_row(row!["User", &env("USER".to_string())]);
        table.printstd();
    }
}

#[cfg(feature = "plain_output")]
pub async fn async_editor(matches: &ArgMatches<'_>) {
    if matches.is_present("editor") {
        println!("Editor:       {}", env("EDITOR".to_string()));
    }
}

#[cfg(feature = "plain_output")]
pub async fn async_shell(matches: &ArgMatches<'_>) {
    if matches.is_present("shell") {
        println!("Shell:        {}", env("SHELL".to_string()));
    }
}

#[cfg(feature = "plain_output")]
pub async fn async_user(matches: &ArgMatches<'_>) {
    if matches.is_present("user") {
        println!("User:         {}", env("USER".to_string()));
    }
}
