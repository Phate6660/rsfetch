use std::fs::File;

pub fn read(file: File) -> Result<String, Box<dyn std::error::Error>> {
    use std::io::Read;
    let mut buf_reader = std::io::BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn line(file: File, line: usize) -> String {
    let contents = read(file).unwrap();
    let file_vector: Vec<&str> = contents.split('\n').collect();
    file_vector[line].to_string()
}

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
