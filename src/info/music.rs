#[cfg(feature = "music")]
pub fn music() -> String {
    use mpd::{Client, Song};
    let mut c = Client::connect("127.0.0.1:6600").unwrap();
    let song: Song = c.currentsong().unwrap().unwrap();
    let na = "N/A".to_string();
    let tit = song.title.as_ref().unwrap();
    let art = song.tags.get("Artist").unwrap_or(&na).to_string();
    let alb = song.tags.get("Album").unwrap_or(&na).to_string();
    let dat = song.tags.get("Date").unwrap_or(&na).to_string();
    let gen = song.tags.get("Genre").unwrap_or(&na).to_string();
    format!("{} - {} ({}) - {} -- {}", art, alb, dat, tit, gen)
}

#[cfg(feature = "nomusic")]
pub fn music() -> String {
    "N/A (music feature must be used to pull in the mpd dependency)".to_string()
}
