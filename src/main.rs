use warp::{filters::fs::file, *};
use json::*;
use std::{ptr::null, *};

#[tokio::main]
async fn main() {
    check_files();

    let site = warp::fs::dir("./webpages");

    /*let api_upload = warp::path!("api" / "v1" / "upload")
        .and(warp::post())
        .and(//i need to have this allow file uploads)
        .and_then(file_upload); */

    let route = site;
        //.or(dir);
        //.or(api_upload);

    warp::serve(route)
        .run(([0, 0, 0, 0], 3000))
        .await;
}

async fn file_upload() {

}

//Create the folder / check every time it is run for satefy c:
fn check_files() -> std::io::Result<()> {
    Ok(if std::fs::File::open("./datafiles/data.json").is_err() {
    std::fs::DirBuilder::new().create("./datafiles");
    assert!(std::fs::metadata("./datafiles").unwrap().is_dir());
    std::fs::File::create("./datafiles/data.json")?;
    println!("Created data");
    } else {
        println!("Loaded config and data");
    })
}