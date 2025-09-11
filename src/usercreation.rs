use futures::io;
use uuid::Uuid;
use random_string::*;
use log::*;
use std::{fs::OpenOptions, io::Write};

static CHARSET: &str = "-ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890_";

pub fn createUser(username: String, password: String) -> io::Result<()> {
    let id = Uuid::new_v4();
    let api_key = generate(40, CHARSET);

    let mut file = OpenOptions::new()
        .append(true)
        .open("./datafiles/accounts.env")?;
    
    writeln!(file, "{}USER={}", username, username);
    writeln!(file, "{}PASSWORD={}", username, password);
    writeln!(file, "{}ID={}", username, id);
    writeln!(file, "{}APIKEY={}", username, api_key);

    Ok(())
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