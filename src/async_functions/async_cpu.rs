use crate::info::cpu::cpu;
use clap::ArgMatches;

#[cfg(feature = "pretty_output")]
pub async fn async_cpu(matches: &ArgMatches<'_>) {
    if matches.is_present("cpu") {
        use prettytable::{format, Table};
        let mut table = Table::new();
        let format = format::FormatBuilder::new()
            .column_separator('│')
            .borders(' ')
            .separators(&[format::LinePosition::Top],
                        format::LineSeparator::new('─', '+', '+', '+'))
            .padding(0, 10)
            .build();
        table.set_format(format);
        table.add_row(row!["CPU", &cpu().trim()]);
        table.printstd();
    }
}

#[cfg(feature = "plain_output")]
pub async fn async_cpu(matches: &ArgMatches<'_>) {
    if matches.is_present("cpu") {
        println!("CPU:          {}", cpu().trim());
    }
}
