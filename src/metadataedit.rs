use std::path::Path;

use log::{info, error};
use multitag::{data::Timestamp, Tag};

//songs
pub fn songmetadata(file: &Path, songtitle: Option<String>, songartist: Option<String>, date: Option<Timestamp>, lyrics: Option<String>, /*cover: Option<String>*/) {
    // TODO : Get & set cover

    let mut tag = Tag::read_from_path(&file).unwrap();

    if let Some(newtitle) = songtitle {
        tag.set_title(&newtitle);
        info!("set song title to: {:?}", newtitle);
        tag.write_to_path(&file).expect("Failed to write to path, please make sure the album exists and hasn't been deleted");
    } else if songtitle.is_none() {
        info!("title has not been edited");
    } else {
        error!("Failed to edit title");
    }

    if let Some(newartist) = songartist {
        tag.set_artist(&newartist);
        info!("Set artist to: {:?}", newartist);
        tag.write_to_path(&file).expect("Failed to write to path, please make sure the album exists and hasn't been deleted");
    } else if songartist.is_none() {
        info!("artist has not been edited");
    } else {
        error!("Failled to edit artist");
    }

    if let Some(newdate) = date {
         tag.set_date(newdate);
         info!("Set new date");
         tag.write_to_path(&file).expect("Failed to write to path, please make sure the album exists and hasn't been deleted");
    } else if date.is_none() {
        info!("date has not been edited");  
    } else {
        error!("Failled to edit date");
    }

    if let Some(newlyrics) = lyrics {
        tag.set_lyrics(&newlyrics);
        info!("Set new lyrics");
        tag.write_to_path(&file).expect("Failed to write to path, please make sure the album exists and hasn't been deleted");
    } else if lyrics.is_none() {
        info!("lyrics have not been edited");
    } else {
        error!("Failled to edit date");
    }
}

pub fn albummetadata(path: &Path, title: Option<String>, artist: Option<String>, date: Option<Timestamp>, /*cover: Option<String>*/) {
    let mut tag = Tag::read_from_path(&path).unwrap();

    if let Some(newtitle) = title {
        tag.set_title(&newtitle);
        info!("set song title to: {:?}", newtitle);
        tag.write_to_path(&path).expect("Failed to write to path, please make sure the album exists and hasn't been deleted");
    } else if title.is_none() {
        info!("title has not been edited");
    } else {
        error!("Failed to edit title");
    }

    if let Some(newartist) = artist {
        tag.set_artist(&newartist);
        info!("Set artist to: {:?}", newartist);
        tag.write_to_path(&path).expect("Failed to write to path, please make sure the album exists and hasn't been deleted");
    } else if artist.is_none() {
        info!("artist has not been edited");
    } else {
        error!("Failled to edit artist");
    }

    if let Some(newdate) = date {
         tag.set_date(newdate);
         info!("Set new date");
         tag.write_to_path(&path).expect("Failed to write to path, please make sure the album exists and hasn't been deleted");
    } else if date.is_none() {
        info!("date has not been edited");  
    } else {
        error!("Failled to edit date");
    }
}