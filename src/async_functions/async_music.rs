use crate::info::music::music;
use clap::ArgMatches;

#[cfg(feature = "pretty_output")]
pub async fn async_music(matches: &ArgMatches<'_>) {
    if matches.is_present("music") {
        use prettytable::{format, Table};
        let mut table = Table::new();
        let format = format::FormatBuilder::new()
            .column_separator('│')
            .borders(' ')
            .separators(&[format::LinePosition::Bottom],
                        format::LineSeparator::new('─', '+', '+', '+'))
            .padding(0, 8)
            .build();
        table.set_format(format);
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
