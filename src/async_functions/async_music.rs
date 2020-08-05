use crate::info::music::music;
use clap::ArgMatches;

pub async fn async_music(matches: &ArgMatches<'_>) {
    if matches.is_present("music") {
        println!("Music:        {}", music());
    }
}
