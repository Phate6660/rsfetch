use nixinfo::music;
use clap::ArgMatches;

#[cfg(feature = "pretty_output")]
pub fn output_music(matches: &ArgMatches) {
    if matches.is_present("music") {
        let mut table = crate::shared_functions::table('│', prettytable::format::LinePosition::Bottom, 8);
        table.add_row(row!["Music", &music().as_str()]);
        table.printstd();
    }
}

#[cfg(feature = "plain_output")]
pub fn output_music(matches: &ArgMatches) {
    if matches.is_present("music") {
        println!("Music:        {}", music());
    }
}
