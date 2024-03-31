// Import input/output library from the standard library.
//   - The `use` statement brings the `io` library into the program's scope from the standard library (`std`).
//   - The `::` syntax indicates that `io` is an associated item of the `std` library.
use std::io;

// Define a `main` function. (A `main` function is required in every executable Rust program. It serves as the program's entry point.)
fn main() {
    // Use `println!` macros to print string values.
    println!("==Number Guessing Game Starts!==");

    println!("Please input your guess:");

    // Create a mutable variable and bind it to an empty string.
    let mut guess = String::new();


    io::stdin() // Use the `stdin()` function to handle a user's standard input from the terminal.
        .read_line(&mut guess) // Get whatever the user types into the standard input and append it to the `guess` string. (Note: `read_line` returns a `Result` value.)
        .expect("Failed to read line"); // Use the `Result` value's `expect()` method to handle any error `read_line` may return.

    // Print out the user's guess.
    println!("You guessed: {guess}");
}