//use warp::*;
use std::{io::Write, *};
use json::*;
use dotenv;
use colored::*;

#[tokio::main]
async fn main() {
    dotenv::from_filename("./datafiles/admin.env").ok();

    check_files();
    println!("\nLoaded data");

    let site = warp::fs::dir("./webpages"); 

    /*let api_upload = warp::path!("api" / "v1" / "upload")
        .and(warp::post())
        .and()
        .and_then(file_upload); */

    let route = site;
        //.or(dir);
        //.or(api_upload);

    warp::serve(route)
        .run(([0, 0, 0, 0], 3000))
        .await;
    println!("\nServer started");
}

async fn file_upload() {
    //
}

//Create the folder / check every time it is run for satefy c:
fn check_files() -> std::io::Result<()> {
    let data = object! {
        music_dir: Null,
        port: Null,
    };

    let data_to_string = data.dump();

    let settings = object! {
        view_without_login: false,
        listen_without_login: false,
        require_login: true,
    };

    let settings_to_string = settings.dump();

    if std::fs::read_dir("./datafiles").is_err() {
        std::fs::DirBuilder::new().create("./datafiles");
        assert!(std::fs::metadata("./datafiles").unwrap().is_dir());
        print!("Created {}", "data folder".truecolor(100, 100, 100));
    }

    if std::fs::File::open("./datafiles/data.json").is_err() {
        let mut datafile = std::fs::File::create("./datafiles/data.json")?;
        datafile.write(data_to_string.as_bytes())?;
        print!(", {},", "data.json".bright_yellow());
    } 

   if std::fs::File::open("./datafiles/settings.json").is_err() {
        let mut settingsfile = std::fs::File::create("./datafiles/settings.json")?;
        settingsfile.write(settings_to_string.as_bytes())?;
        print!(" {}", "settings.json".bright_yellow());
    };

    if std::fs::File::open("./datafiles/admin.env").is_err() {
        std::fs::File::create("./datafiles/admin.env")?;
        print!(", {}", "admin.env".cyan());
    }

    Ok(())
}