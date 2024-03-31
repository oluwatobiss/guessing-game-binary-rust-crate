// Import input/output library from the standard library.
//   - The `use` statement brings the `io` library into the program's scope from the standard library (`std`).
//   - The `::` syntax indicates that `io` is an associated item of the `std` library.
use std::io;
use rand::Rng; // `Rng` is a trait that must be in scope to use random number generators' methods.
use std::cmp::Ordering;

// Define a `main` function. (A `main` function is required in every executable Rust program. It serves as the program's entry point.)
fn main() {
    // Use `println!` macros to print string values.
    println!("==Number Guessing Game Starts!==");

    // Generate a random number between 1 and 100 inclusive.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Debug code: Print the secret number.
    println!("The secret number is: {secret_number}");

    loop {
        println!("Guess the computer's choice.");

        println!("Tip: A number from 1 to 100.");

        // Create a mutable variable and bind it to an empty string.
        let mut guess = String::new();

        io::stdin() // Use the `stdin()` function to handle a user's standard input from the terminal.
            .read_line(&mut guess) // Get whatever the user types into the standard input and append it to the `guess` string. (Note: `read_line` returns a `Result` value.)
            .expect("Failed to read line"); // Use the `Result` value's `expect()` method to handle any error `read_line` may return.

        // Convert the `guess` string to a real number type.
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // Print out the user's guess.
        println!("You guessed: {guess}");

        // Match the comparison of guess and secret_number with the arm's patterns.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}