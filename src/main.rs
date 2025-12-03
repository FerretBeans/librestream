pub mod metadataedit;
pub mod usercreation;

use std::{path::Path, sync::{Arc, Mutex}, vec};

use metadataedit::*;
use multitag::data::Timestamp;
use usercreation::*;

use colored::*;
use dotenv;
use ftail::Ftail;
use futures::TryStreamExt;
use log::{LevelFilter, error, info};
use multer::bytes::BufMut;
use serde::Deserialize;
use serde_json::*;
use warp::{Filter, filters::multipart::{FormData, Part}, reject::Rejection, reply::Reply};

// This is for the creation of the objects that allows me to write to the
// functions easier
#[derive(Deserialize)]
struct Metadata {
    path: Box<Path>,
    title: Option<String>,
    artist: Option<String>,
    lyrics: Option<String>,
    year: Option<i32>,
    month: Option<u8>,
    day: Option<u8>,
}

#[derive(Deserialize)]
struct Userdata {
    un: String,
    pw: String,
    token: String
}

#[derive(Deserialize)]
struct Token {
    token: String
}

#[tokio::main]
async fn main() {
    //anything for other funcs
    let tokens = Arc::new(Mutex::new(Vec::<String>::new()));
    let filter = warp::any().map(move || tokens.clone());

    if std::fs::read_dir("./logs").is_err() {
        std::fs::DirBuilder::new()
            .create("./logs")
            .expect("Failed to create path");
        assert!(std::fs::metadata("./logs").unwrap().is_dir());
        info!("Created {}", "Logs folder".bright_green());
    }

    //Initialise logger
    Ftail::new()
        .console(LevelFilter::Debug)
        .daily_file(std::path::Path::new("logs"), LevelFilter::Debug)
        .init()
        .expect("Log failed to start");

    info!("Started logging");

    //Run check files to check if any major files exist
    check_files().expect("Failed to check files");

    //USE HERE FOR TESTING FUNCTIONS

    //Inject .env file
    run_dotenv();

    //Get the port the admin wants to use for the server, default is 22501
    let read_json = std::fs::read_to_string("./datafiles/settings.json").expect("Failed to read settings.json");
    let getport: Value = serde_json::from_str(&read_json).expect("Failed to read settings.json; Read Port");
    let port = &getport["server_port"];
    let portu64 = port
        .as_u64()
        .expect("Port must be between 1 - 65535");
    let portu16 = u16::try_from(portu64)
        .expect("Couldn't convert to u16 or you've chosen a number thats bigger than 65535");

    //Webui pages
    let blocked = warp::path("data") // Prevents the access of accounts.env
        .and(warp::any())
        .map(|| warp::reply::with_status("Forbidden", warp::http::StatusCode::FORBIDDEN));

    let files = warp::path("files")
        .and(warp::any())
        .and(warp::fs::dir("./webpages"));

    let logs = warp::path("logs").and(warp::fs::dir("./logs"));
    let settingsfile = warp::path!("data").and(warp::fs::dir("./datafiles/settings.json"));
    let site = warp::path::end().and(warp::fs::file("./webpages/index.html"));
    let settings = warp::path!("settings").and(warp::fs::file("./webpages/settings.html"));
    
    let blocked_page =
        warp::path!("disallowed")
        .and(warp::fs::file("./webpages/blocked/blockedpage.html"));

    let login_page =
        warp::path!("api" / "v1" / "login")
        .and(warp::fs::file("./webpages/login-page.html"));

    let api_upload = warp::path!("api" / "v1" / "upload")
        .and(warp::post())
        .and(warp::multipart::form().max_length(u64::MAX))
        .and_then(file_upload);

    let user_creation = warp::path!("api" / "v1" / "usercreate")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(user_data);

    let songmetadata = warp::path!("api" / "v1" / "songdata")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(songmetadataedit);

    let albummetadata = warp::path!("api" / "v1" / "albumdata")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(albummetadataedit);

    let token = warp::path!("api" / "v1" / "token")
        .and(warp::post())
        .and(warp::body::json())
        .and(filter)
        .and_then(web_session_token);

    //actually allow the sites to be accessed
    let route = blocked
        .or(site)
        .or(logs)
        .or(login_page)
        .or(blocked_page)
        .or(settings)
        .or(settingsfile)
        .or(user_creation)
        .or(files)
        .or(songmetadata)
        .or(albummetadata)
        .or(token)
        .or(api_upload);

    info!("Running on port {}", port);
    warp::serve(route).run(([0, 0, 0, 0], portu16)).await;
}

//Upload the file to the server
//highkey stole this code uh im gonna code my own at some point
async fn file_upload(form: FormData) -> std::result::Result<impl Reply, Rejection> {
    // Get the directory where music is to be stored
    let read_json =
        std::fs::read_to_string("./datafiles/settings.json").expect("Failed to read settings.json");
    let getdir: Value =
        serde_json::from_str(&read_json).expect("Failed to read settings.json; Directory");
    let dir = getdir["music_dir"]
        .as_str()
        .expect("music_dir must be a string");

    let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        error!("form error: {}", e);
        warp::reject()
    })?;

    for p in parts {
        if p.name() == "file" {
            let content_type = p.content_type();

            let file_ending;
            match content_type {
                Some(file_type) => match file_type {
                    "audio/mpeg" => file_ending = "mp3",
                    "audio/wav" => file_ending = "wav",
                    "audio/ogg" => file_ending = "ogg",
                    "audio/flac" => file_ending = "flac",
                    "audio/aac" => file_ending = "aac",
                    "audio/opus" => file_ending = "opus",
                    v => {
                        error!("invalid file type found: {}", v);
                        return Err(warp::reject());
                    }
                },
                None => {
                    error!("file type could not be determined");
                    return Err(warp::reject());
                }
            }

            // Move filename out *before* the stream
            let base_name = p.filename().unwrap_or("upload").to_string();

            let value = p
                .stream()
                .try_fold(Vec::new(), |mut vec, data| {
                    vec.put(data);
                    async move { Ok(vec) }
                })
                .await
                .map_err(|e| {
                    error!("reading file error: {}", e);
                    warp::reject()
                })?;

            let file_name = format!("{}/{}.{}", dir, base_name, file_ending);

            std::fs::write(&file_name, value).map_err(|e| {
                error!("error writing file: {}", e);
                warp::reject()
            })?;

            info!("uploaded file: {}", file_name);
        }
    }

    Ok("Upload received")
}

//Create the folder / check every time it is run for satefy c:
fn check_files() -> std::io::Result<()> {
    let settings = json!({
        "music_dir": "/var/music",
        "server_port": 22501,
        "view_without_login": false,
        "listen_without_login": false,
        "require_login": true,
        "allow_create_user": true,
    });

    //make the json a string to import into a file
    if std::fs::read_dir("./datafiles").is_err() {
        std::fs::DirBuilder::new()
            .create("./datafiles")
            .expect("Failed to create path");
        assert!(std::fs::metadata("./datafiles").unwrap().is_dir());
        info!("Created {}", "data folder".truecolor(100, 100, 100));
    }

    if std::fs::File::open("./datafiles/settings.json").is_err() {
        let settingsfile = std::fs::File::create("./datafiles/settings.json")?;
        serde_json::to_writer_pretty(settingsfile, &settings)?;
        info!("Created {}", "settings.json".bright_yellow());
    }

    if std::fs::read_dir("./data").is_err() {
        std::fs::DirBuilder::new()
            .create("./data")
            .expect("Failed to create path");
        assert!(std::fs::metadata("./data").unwrap().is_dir());
        info!("Created {}", "data folder".truecolor(100, 100, 100));
    }

    if std::fs::File::open("./data/accounts.env").is_err() {
        std::fs::File::create("./data/accounts.env")?;
        info!("Created {}", "accounts.env".cyan());
    }

    info!("Data loaded");

    Ok(())
}

async fn user_data(body: Userdata) -> std::result::Result<impl warp::Reply, warp::Rejection> {
    let read_json =
        std::fs::read_to_string("./datafiles/settings.json").expect("Failed to read settings.json");
    let settings: Value = serde_json::from_str(&read_json).expect("Failed to read settings.json");

    let allowusercreate = &settings["allow_create_user"];

    if allowusercreate == true {
        create_user(body.un.clone(), body.pw, body.token).expect("Failed to create user");
        info!("A user has been created");
        let message = format!("User {} has been created", body.un);
        Ok(warp::reply::with_status(message,warp::http::StatusCode::ACCEPTED))
    } else {
        Ok(warp::reply::with_status("User creation is disabled".to_string(),warp::http::StatusCode::FORBIDDEN))
    }
}

fn run_dotenv() {
    dotenv::from_path("./data/accounts.dev").ok();
    info!("Loaded the {}", "enviornment file".bright_blue());
}

async fn songmetadataedit(mut data: Metadata) -> std::result::Result<impl warp::Reply, warp::Rejection> {
    // TODO : cover art
    let path = data.path.clone();

    //for individual songs
    if !path.exists() {
        return Ok(warp::reply::with_status("File is emtpy, how the did you achieve this????",warp::http::StatusCode::BAD_REQUEST));
    }

    let year = data.year.unwrap_or(0000);
    let month = data.month;
    let day = data.day;
    let date = Timestamp { year, month, day, hour: None, minute: None, second: None };
        
    if let Some(newartist) = data.artist { data.artist = Some(newartist); }
    if let Some(newlyrics) = data.lyrics { data.lyrics = Some(newlyrics); }
    if let Some(newtitle) = data.title { data.title = Some(newtitle); }

    songmetadata(&data.path, data.title, data.artist, Some(date), data.lyrics);
    Ok(warp::reply::with_status("Edited metadata",warp::http::StatusCode::OK))
}

async fn albummetadataedit(mut data: Metadata) -> std::result::Result<impl warp::Reply, warp::Rejection> {
    // TODO : have cover inside album folder which then is gotten and placed as cover art for album
    return Ok("0");
}

async fn web_session_token(token: Token, vec: Arc<Mutex<Vec<String>>>) -> std::result::Result<impl warp::Reply, warp::Rejection> {
    // TODO : Retrive session token from website and make it so those tokens can or cant be used to upload music
    let mut tokens = vec.lock().unwrap();
    tokens.push(token.token.clone());
    println!("{:?}", tokens);

    Ok(warp::reply::with_status("token accepted", warp::http::StatusCode::OK))
}