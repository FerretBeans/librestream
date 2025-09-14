use log::{info, error};
use multitag::{data::Timestamp, Tag};

pub fn album_metadata(file: String, title: Option<&str>, artist: Option<&str>, date: Option<Timestamp>, lyrics: Option<&str>) {
    // TODO : Get cover - uses rgb thingy so idkkkkk
    // TODO : when new song name is set send that to the thingy to set it
    // TODO : send the info to the website

    let mut tag = Tag::read_from_path(&file).unwrap();

    //let cover = tag.get_album_info();

    if let Some(newtitle) = title {
        tag.set_title(newtitle);
        info!("set song title to: {:?}", newtitle);
        tag.write_to_path(&file).expect("Failed to write to path, please make sure the album exists and hasn't been deleted");
    } else if title.is_none() {
        // Do nothing
    } else {
        error!("Failed to edit title");
    }

    if let Some(newartist) = artist {
        tag.set_artist(newartist);
        info!("Set artist to: {:?}", newartist);
        tag.write_to_path(&file).expect("Failed to write to path, please make sure the album exists and hasn't been deleted");
    } else if artist.is_none() {
        // Do nothing
    } else {
        error!("Failled to edit artist");
    }

    if let Some(newdate) = date {
         tag.set_date(newdate);
         info!("Set new date");
         tag.write_to_path(&file).expect("Failed to write to path, please make sure the album exists and hasn't been deleted");
    } else if date.is_none() {
        // Do nothing
    } else {
        error!("Failled to edit date");
    }

    if let Some(newlyrics) = lyrics {
        tag.set_lyrics(newlyrics);
        info!("Set new lyrics");
        tag.write_to_path(&file).expect("Failed to write to path, please make sure the album exists and hasn't been deleted");
    } else if lyrics.is_none() {
        // Do nothing
    } else {
        error!("Failled to edit date");
    }

}

// pub fn song_metadata(file: String) {
//     // TODO : get song data and write to it :3
//     //let tag = Tag::read_from_path(file).unwrap();

//     // let artist = tag.artist();
//     // let date= tag.date();
//     // let title= tag.title();
//     // let lyrics = tag.lyrics();
//     //let cover = tag.get_album_info();
// }