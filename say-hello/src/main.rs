// Problem Statement:
// The "Hello, World" program is the first program you learn to write in many languages, but doesn't involve any input.
// So create a program that prompts for your name and prints a greeting using your name.

// Example Output:
// What is your name? Jeff
// Hello, Jeff, nice to meet you!

// Constraint:
// Keep the input, string concatenation, and output separate.

use std::io::{self, Write};

/// function that retrieves user input
fn get_name() -> String {
    let mut buffer = String::new();
    print!("What is your name? ");
    io::stdout().flush().expect("There was an I/O error and the buffer could not be flushed.");
    io::stdin().read_line(&mut buffer).expect("Unable to read your name...");
    buffer.trim().to_string()
}

/// function that builds the output string
fn build_greeting(name: String) -> String {
    let mut greeting = "Hello, ".to_owned();
    greeting.push_str(&name[..]);
    greeting.push_str(", nice to meet you!");
    greeting
}
/// function that displays the output string
fn print_greeting(greeting: String) {
    println!("{}", greeting);
}
/// main program
fn main() {
    loop {
        let name = get_name();
        if name.is_empty() {
            println!("Sorry, name cannot be empty, please try again.")
        } else {
            let greeting = build_greeting(name);
            print_greeting(greeting);
            break;
        }
    }
}

// tests
// none (program can be tested trivially by running it)
