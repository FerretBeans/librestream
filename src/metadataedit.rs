use log::{info, warn};
use multitag::*;

pub fn album_metadata(file: String, title: Option<&str>) {
    // TODO : write code to read and metadata to send to the page
    // TODO : Get cover - uses rgb thingy so idkkkkk
    // TODO : when new song name is set send that to the thingy to set it

    let tag = Tag::read_from_path(file).unwrap();

    let artist = tag.artist();
    let date= tag.date();
    let songtitle= tag.title();
    let lyrics = tag.lyrics();
    //let cover = tag.get_album_info();
}

pub fn song_metadata(file: String) {
    // TODO : get song data and write to it :3
    let tag = Tag::read_from_path(file).unwrap();

    let artist = tag.artist();
    let date= tag.date();
    let title= tag.title();
    let lyrics = tag.lyrics();
    //let cover = tag.get_album_info();
}