use std::fs;
use std::fs::*;
use std::io;
use std::io::*;

fn save_password() -> io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .open("data/passwords.dat")?;

    Ok(())
}