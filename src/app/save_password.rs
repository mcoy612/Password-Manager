use std::fs;
use std::fs::*;
use std::io;
use std::io::*;

use crate::tools::prompt;

fn save_password(application: &str, encrypted_password: &str) -> io::Result<()> {
    let mut password_file = get_password_file()?;

    Ok(())
}

fn get_password_file() -> io::Result<File> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("data/passwords.dat")?;

    Ok(file)
}

fn replace_password() -> bool {
    println!("Password already exists for this application.");
    let yes = prompt::yn();

    yes
}