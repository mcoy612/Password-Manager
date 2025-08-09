mod generate_password;
mod save_password;
mod get_password;

use std::io;
use std::io::Write;

fn main() {
    let mut quit: bool = false;
    while !quit {
        println!("Choose Option");
        println!("1. Save Password");
        println!("2. Get Password");
        println!("3. Generate Password");

        let mut menu_option: String = String::new();
        let mut done: bool = false;
        while !done {
            print!("Option: ");
            io::stdout().flush().unwrap();
            if let Err(error) = io::stdin().read_line(&mut menu_option) {
                println!("Error: {error}");
                menu_option = String::new();
            }

            menu_option = menu_option.trim().to_uppercase();
            done = true
        }

        if menu_option=="" {
            quit = true
        } else if menu_option=="1"||menu_option=="SAVE PASSWORD" {
            unimplemented!();
        } else if menu_option=="2"||menu_option=="GET PASSWORD" {
            unimplemented!();
        } else if menu_option=="3"||menu_option=="GENERATE PASSWORD" {
            unimplemented!();
        } else {
            println!("Not a valid option");
        }
    }
}
