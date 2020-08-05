use std::fs::File;
use std::io::{BufReader, Read};

pub fn read(file: File) -> Result<String, Box<dyn std::error::Error>> {
    let mut buf_reader = BufReader::new(file);
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
pub fn table(pad: usize) -> Table {
    use prettytable::format;
    let mut table = Table::new();
    let format = format::FormatBuilder::new()
        .column_separator('|')
        .borders(' ')
        .separators(
            &[format::LinePosition::Intern],
            format::LineSeparator::new('-', '+', '+', '+'),
        )
        .padding(0, pad)
        .build();
    table.set_format(format);
    table
}
