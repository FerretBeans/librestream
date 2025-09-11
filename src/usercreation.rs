use uuid::Uuid;
use random_string::*;
use log::*;
use std::io;

static CHARSET: &str = "-ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890_";

pub fn createUser(username: String, password: String) {
    let id = Uuid::new_v4();
    let api_key = generate(40, CHARSET);
    
    writeln!("{}USER: {}", "{} {}", username, username);
    writeln!("");
    writeln!("");
    writeln!("");
}

pub fn deleteUser() {
    //TODO : figure out to delete, as i want to delete uuid and api key but are random gend
}

pub fn updateUser() {
    //TODO : allow all users to change username and pw
}

//not really used outside the thingy
fn reload_dotenv() {
    //reload the env var
}