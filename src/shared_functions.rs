#[cfg(feature = "pretty_output")]
use prettytable::Table;
#[cfg(feature = "pretty_output")]
pub fn table(border: char, pos: prettytable::format::LinePosition, pad: usize) -> Table {
    let mut table = Table::new();
    let format = prettytable::format::FormatBuilder::new()
        .column_separator('│')
        .borders(border)
        .separators(
            &[pos],
            prettytable::format::LineSeparator::new('─', '+', '+', '+'),
        )
        .padding(1, pad)
        .build();
    table.set_format(format);
    table
}
