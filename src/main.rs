//pub mod usercreation;

use log::*;
use serde_json::*;
use colored::*;
use warp::{filters::multipart::{FormData, Part}, reject::Rejection, reply::Reply, Filter};
use multer::bytes::{Buf, BufMut};
use futures::TryStreamExt;
use dotenv;
use env_logger::*;
use ftail::Ftail;

//use usercreation::*;

#[tokio::main]
async fn main() {
    if std::fs::read_dir("./logs").is_err() {
        std::fs::DirBuilder::new().create("./logs");
        assert!(std::fs::metadata("./logs").unwrap().is_dir());
        info!("Created {}", "Logs folder".bright_green());
    }

    //Initialise logger
    Ftail::new()
        .console(LevelFilter::Debug)
        .daily_file(std::path::Path::new("logs"), LevelFilter::Debug)
        .init();

    info!("Started logging");

    /*env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .init();*/

    //Run check files to check if any major files exist
    check_files();

    //Inject .env file
    run_dotenv();

    //Get the port the admin wants to use for the server, default is 3000
    let read_json = std::fs::read_to_string("./datafiles/settings.json").expect("Failed to read settings.json");
    let getport: Value = serde_json::from_str(&read_json).expect("Failed to read settings.json; Read Port");
    let port = &getport["server_port"];
    let portu64 = port.as_u64().expect("Port must be between 1 - 65535");
    let portu16 = u16::try_from(portu64).expect("Couldn't convert to u16");
    
    //Webui pages
    let site = warp::path::end()
        .and(warp::fs::file("./webpages/index.html"));

    let settings = warp::path!("settings")
        .and(warp::fs::file("./webpages/settings.html"));

    let blocked_page = warp::path!("disallowed")
        .and(warp::fs::file("./webpages/blocked/blockedpage.html"));

    let login_page = warp::path!("api" / "v1" / "login")
        .and(warp::fs::file("./webpages/login-page.html"));

    let api_upload = warp::path!("api" / "v1" / "upload")
        .and(warp::post())
        .and(warp::multipart::form().max_length(u64::MAX))
        .and_then(file_upload);

    /*let user_creation = warp::path!("api" / "v1" / "usercreate")
        .and(warp::post())
        .and_then(createUser({}, {}));*/

    //actually allow the sites to be accessed
    let route = site
        .or(login_page)
        .or(blocked_page)
        .or(settings)
        .or(api_upload);

    info!("Running on port {}", port);
    warp::serve(route)
        .run(([0, 0, 0, 0], portu16))
        .await;
}

//Upload the file to the server
async fn file_upload(form: FormData) -> std::result::Result<impl Reply, Rejection> {
    // Get the directory where music is to be stored
    let read_json = std::fs::read_to_string("./datafiles/settings.json").expect("Failed to read settings.json");
    let getdir: Value = serde_json::from_str(&read_json).expect("Failed to read settings.json; Directory");
    let dir = getdir["music_dir"].as_str().expect("music_dir must be a string");

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
                "audio/mpeg" => {
                    file_ending = "mp3";
                }
                "audio/wav" => {
                    file_ending = "wav";
                }
                "audio/ogg" => {
                    file_ending = "ogg"
                }
                "audio/flac" => {
                    file_ending = "flac"
                }
                "audio/aac" => {
                    file_ending = "aac"
                }
                "audio/opus" => {
                    file_ending = "opus"
                }
                v => {
                    eprintln!("invalid file type found: {}", v);
                    return Err(warp::reject());
                }
            },
            None => {
                eprintln!("file type could not be determined");
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
        "server_port": 3000,
        "view_without_login": false,
        "listen_without_login": false,
        "require_login": true,
        "upload_require_login": true,
        "allow_create_user": false,
    });

    //make the json a string to import into a file
    if std::fs::read_dir("./datafiles").is_err() {
        std::fs::DirBuilder::new().create("./datafiles");
        assert!(std::fs::metadata("./datafiles").unwrap().is_dir());
        info!("Created {}", "data folder".truecolor(100, 100, 100));
    }

    if std::fs::File::open("./datafiles/settings.json").is_err() {
        let settingsfile = std::fs::File::create("./datafiles/settings.json")?;
        serde_json::to_writer_pretty(settingsfile, &settings)?;
        info!("Created {}", "settings.json".bright_yellow());
    }

    if std::fs::File::open("./datafiles/accounts.env").is_err() {
        std::fs::File::create("./datafiles/accounts.env")?;
        info!("Created {}", "accounts.env".cyan());
    }

    info!("Data loaded");

    Ok(())
}

fn run_dotenv() {
    dotenv::from_path("./datafiles/accounts.dev").ok();
    info!("Loaded the {}", "enviornmental file".bright_blue());
}