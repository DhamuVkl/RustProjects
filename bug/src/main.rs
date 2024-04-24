use std::io::{self, BufRead};

fn main() {
    println!("Hello, what's your name?");
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin
        .lock()
        .read_line(&mut buffer)
        .expect("Unable to read stdin");

    if buffer.trim() == "Dhamu" {
        // Removed newline character from comparison
        println!("Hello Dhamu");
    } else {
        println!("You aren't an authorized user.");
    }
}
