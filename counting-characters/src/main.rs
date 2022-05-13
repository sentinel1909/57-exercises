// Problem Statement:
// Create a program that prompts for an input string and displays output that shows the input string and the number
// of characters the string contains.

// Example Output:
// What is the input string? Homer
// Homer has 5 characters

// Constraint:
// Be sure the output contains the original string.
// Use a single output statement to construct the output.
// Use a built-in function of the programming language to determine the length of the string.

use std::io::{self, Write};

// function to get the user input
fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

// main program
fn main() {
    loop {
        print!("What is the input string? ");
        io::stdout().flush().unwrap();
        match get_input() {
            Ok(input) => {
                let length = input.len();
                if length > 1 {
                    println!("{} has {} characters", input, length);
                    break;
                } else {
                    println!("Please enter a string.");
                }                
            }
            Err(_error) => println!("There was an error.")
        }        
    } 
}

// tests
// none (program can be tested trivially by running it)