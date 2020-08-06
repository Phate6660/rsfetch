use crate::info::music::music;
use clap::ArgMatches;

#[cfg(feature = "pretty_output")]
pub async fn async_music(matches: &ArgMatches<'_>) {
    if matches.is_present("music") {
        let mut table = crate::shared_functions::table('│', prettytable::format::LinePosition::Bottom, 8);
        table.add_row(row!["Music", &music().as_str()]);
        table.printstd();
    }
}

#[cfg(feature = "plain_output")]
pub async fn async_music(matches: &ArgMatches<'_>) {
    if matches.is_present("music") {
        println!("Music:        {}", music());
    }
}
