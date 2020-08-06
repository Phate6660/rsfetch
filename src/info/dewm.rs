use std::env;
use std::fs::{metadata, File};

fn de() -> String {
    env::var("XDG_DESKTOP_SESSION")
        .or_else(|_| env::var("XDG_CURRENT_DESKTOP"))
        .or_else(|_| env::var("DESKTOP_SESSION"))
        .unwrap_or_else(|_| "N/A".to_string())
}

fn wm() -> String {
    let path = format!("{}/.xinitrc", env::var("HOME").unwrap());
    if metadata(&path).is_ok() {
        let file = File::open(&path).unwrap();
        let contents = crate::shared_functions::read(file).unwrap();
        let line = contents.lines().last().unwrap();
        line.split(' ').last().unwrap().to_string()
    } else {
        "N/A (could not open $HOME/.xinitrc)".to_string()
    }
}

pub fn environment() -> String {
    let de = de();
    if de == "N/A" {
        wm()
    } else {
        de
    }
}
