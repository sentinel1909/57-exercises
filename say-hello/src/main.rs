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
    print!("What is your name? ");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer
}
/// function that builds the output string
fn build_greeting(name: String) -> String {
    let mut greeting = "Hello, ".to_owned();
    greeting.push_str(&name[..].trim());
    greeting.push_str(", nice to meet you!");
    greeting
}
/// function that displays the output string 
fn print_greeting(greeting: String) {
    println!("{}", greeting);
}
/// main program
fn main() {
    print_greeting(build_greeting(get_name()));

}

// tests
// none (program can be tested trivially by running it)
