use crate::models::user::User;
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
    sync::Mutex,
};

use anyhow::Result;
use lazy_static::lazy_static;
// Import anyhow::Result

/// Path to the JSON file acting as the database.
const DB_PATH: &str = "db/users.json";

/// In-memory store protected by a Mutex for thread-safe access.
lazy_static! {
    pub static ref USERS: Mutex<Vec<User>> = Mutex::new(load_users().unwrap_or_default());
}

/// Loads users from the JSON file.
fn load_users() -> Result<Vec<User>> {
    // Change return type
    if !Path::new(DB_PATH).exists() {
        // If the file doesn't exist, create it with an empty array
        let file = File::create(DB_PATH)?;
        serde_json::to_writer(file, &Vec::<User>::new())?;
    }

    let mut file = File::open(DB_PATH)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let users = serde_json::from_str(&data)?;
    Ok(users)
}

/// Saves the current state of users to the JSON file.
fn save_users(users: &Vec<User>) -> Result<()> {
    // Change return type
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(DB_PATH)?;
    serde_json::to_writer_pretty(file, users)?;
    Ok(())
}

/// Adds a new user to the database.
pub fn add_user(user: User) -> Result<()> {
    // Change return type
    let mut users = USERS.lock().unwrap();
    users.push(user);
    save_users(&users)
}

/// Retrieves all users from the database.
pub fn get_users() -> Vec<User> {
    let users = USERS.lock().unwrap();
    users.clone()
}
