use std::{convert::Infallible, io::Write, usize};
use json::*;
use colored::*;
use warp::{filters::multipart::FormData, Filter};
use multer::{bytes::Buf, *};
use bytes::Bytes;
use futures_util::{stream::{once, Stream}, StreamExt};
use multer::Multipart;

#[tokio::main]
async fn main() {
    //Run check files to check if any major files exist
    check_files();
    println!("\nLoaded data");

    //Get the port the admin wants to use for the server, default is unknown atm bcs i need to choose one
    let read_json = std::fs::read_to_string("./datafiles/settings.json").unwrap();
    let getport = json::parse(&read_json).unwrap();
    let port = getport["server_port"].as_u32().unwrap();

    //Get the directory where music is to be stored
    let read_json = std::fs::read_to_string("./datafiles/settings.json").unwrap();
    let getdir = json::parse(&read_json).unwrap();
    let dir = getdir["music_dir"].clone();
    
    //Site settings
    let site = warp::path::end()
        .and(warp::fs::dir("./webpages/index.html"));

    let otherpages = warp::fs::dir("./webpages");

    let blocked_page = warp::path!("disallowed")
        .and(warp::fs::file("./webpages/blocked/blockedpage.html"));

    let login_page = warp::path!("api" / "v1" / "login")
        .and(warp::fs::file("./webpages/login-page.html"));

    let api_upload = warp::path!("api" / "v1" / "upload")
        .and(warp::post())
        .and(warp::multipart::form().max_length(u64::MAX))
        .and_then(move |form| file_upload(form, dir.to_string()));

    //actually allow the sites to be accessed
    let route = site
        .or(login_page)
        .or(blocked_page)
        .or(otherpages)
        //.or(dir);
        .or(api_upload);
    
    println!("Running on port {}", port);
    warp::serve(route)
        .run(([0, 0, 0, 0], port as u16))
        .await;
}

//Upload the file to the server
async fn file_upload(mut form: warp::multipart::FormData, upload_directory: String) -> std::result::Result<impl warp::Reply, Infallible> {
    let mut file_count = 0;

    while let Some(part) = form.next().await {
        match part {
            Ok(part) => {
                let filename = part.filename().unwrap_or_else(|| "uploaded_file".into());
                let filepath = format!("{}/{}", upload_directory, filename);
                
                let mut file = std::fs::File::create(filepath).unwrap();

                let mut stream = part.stream();
                while let Some(chunk) = stream.next().await {
                    match chunk {
                        Ok(data) => {
                            file.write_all(data.chunk()).unwrap();
                        },
                        Err(e) => {
                            eprintln!("Error reading chunk: {:?}", e);
                        }
                    }
                }
                file_count += 1;
            },
            Err(e) => {
                eprintln!("Error processing part: {:?}", e);
            }
        }
    }

    Ok(warp::reply::with_status(format!("Uploaded {} file(s)", file_count), warp::http::StatusCode::OK))
}

//Create the folder / check every time it is run for satefy c:
fn check_files() -> std::io::Result<()> {
    let settings = object! {
        music_dir: "/var/music",
        server_port: 3000,
        view_without_login: false,
        listen_without_login: false,
        require_login: true,
        upload_require_login: true,
    };

    let settings_to_string = settings.dump();

    if std::fs::read_dir("./datafiles").is_err() {
        std::fs::DirBuilder::new().create("./datafiles");
        assert!(std::fs::metadata("./datafiles").unwrap().is_dir());
        print!("Created {}", "data folder".truecolor(100, 100, 100));
    }

   if std::fs::File::open("./datafiles/settings.json").is_err() {
        let mut settingsfile = std::fs::File::create("./datafiles/settings.json")?;
        settingsfile.write_all(settings_to_string.as_bytes())?;
        print!(" {}", "settings.json".bright_yellow());
    }

    if std::fs::File::open("./datafiles/accounts.env").is_err() {
        std::fs::File::create("./datafiles/accounts.env")?;
        print!(", {}", "accounts.env".cyan());
    }

    Ok(())
}