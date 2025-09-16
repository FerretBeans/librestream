use colored::Colorize;
use futures::io::{self};
use uuid::Uuid;
use random_string::*;
use log::*;
use warp::reply::with_status;
use std::fs::{read_to_string, File, OpenOptions};
use std::io::Write;

static CHARSET: &str = "-ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890_";

pub fn create_user(username: String, password: String) -> io::Result<()> {
    let id = Uuid::new_v4();
    let api_key = generate(40, CHARSET);

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("./data/accounts.env")?;

    let fileread = read_to_string("./data/accounts.env").unwrap();
    let usernamekey = format!("{}USER={}", username, username);

    let mut userexists = false;

    for line in fileread.lines() {
        if line == usernamekey {
            userexists = true;
            with_status("User already exists!", warp::http::StatusCode::CONFLICT);
            warn!("User already exists");
            break;
        } 
    }

    if !userexists {
        info!("User {} has been created", username.bright_purple());
        writeln!(file, "{}USER={}", username, username)?;
        writeln!(file, "{}PASSWORD={}", username, password)?;
        writeln!(file, "{}ID={}", username, id)?;
        writeln!(file, "{}APIKEY={}", username, api_key)?;
        writeln!(file, "{}ISADMIN=FALSE", username)?;
    } else {
        warn!("user {} still exists", username.bright_purple());
        with_status("User already exists", warp::http::StatusCode::CONFLICT);
    }
    
    reload_dotenv();
    Ok(())
}

pub fn delete_user(username: String, password: String) {
    // TODO : figrued out as i can just named them all {username}user {username}password etc but u cant see that :3 so when u do, just read every line that includes {username}
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("./data/accounts.env");



    reload_dotenv();
}

pub fn upload_profile_picture(profilepicture: Option<&File>) {
    // TODO : handle profile picture and if dont set one have a default

}

pub fn update_user(username: String, newusername: Option<String>, password: String, oldpassword: Option<String>) {
    // TODO : allow all users to change username and pw get {username}
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("./data/accounts.env");

    reload_dotenv();
}

//not really used outside the thingy
fn reload_dotenv() {
    //reload the env var
    match dotenvy::from_filename_override("./data/accounts.env"){
        Ok(_) => info!("Environment file reloaded"),
        Err(e) => error!("Environment file didn't reload:\n {}", e)
    }
}