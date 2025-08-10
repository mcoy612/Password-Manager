use std::io;
use std::io::Write;

pub fn read_line_prompt() -> String {
    unimplemented!();
}

pub fn yn(prompt: &str) -> bool {
    let mut input = String::new();
    let mut yes = false; // Initial value doesn't matter
    while input.is_empty() {
        print!("{prompt} (Y/N): ");
        io::stdout().flush().unwrap();
        if let Err(error) = io::stdin().read_line(&mut input) {
            input.clear();
            println!("Error: {error}");
            continue;
        }
        input = input.trim().to_uppercase();

        if input=="YES"||input=="Y" {
            yes = true;
            break;
        } else if input=="NO"||input=="N" {
            yes = false;
            break;
        } else {
            input.clear();
        }
    }
    yes
}