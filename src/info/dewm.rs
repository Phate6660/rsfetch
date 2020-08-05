use std::env;
use std::fs::{metadata, File};
use std::io::{BufReader, Read};

fn de() -> String {
    env::var("XDG_DESKTOP_SESSION")
        .or_else(|_| env::var("XDG_CURRENT_DESKTOP"))
        .or_else(|_| env::var("DESKTOP_SESSION"))
        .unwrap_or("N/A".to_string())
}

fn wm() -> Result<String, Box<dyn std::error::Error>> {
    let path = format!("{}/.xinitrc", env::var("HOME").unwrap());
    if metadata(&path).is_ok() {
        let file = File::open(&path).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        let line = contents.lines().last().unwrap();
        Ok(line.split(' ').last().unwrap().to_string())
    } else {
        Ok("N/A (could not open $HOME/.xinitrc)".to_string())
    }
}

pub fn environment() -> String {
    let de = de();
    if de == "N/A" {
        wm().unwrap()
    } else {
        de
    } 
}
