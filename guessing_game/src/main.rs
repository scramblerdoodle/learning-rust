// Rust syntax for imports: `use {namepace}::{lib name}`
// Standard Input-Output lib
use std::io;

// String comparison
use std::cmp::Ordering;

// Random Number Generator
use rand::Rng;

// All Rust codes need a `main` function as the entrypoint
fn main() {
    // Print macro
    println!("Guess the number!");

    // Generate random number from rand
    // 1..=100 means "from 1 to 100 inclusive", I think it's a sort of iterator
    // `cargo doc --open` can open the project's documentation, which includes the dependencies
    let correct = rand::thread_rng().gen_range(1..=100);

    // while loop in Rust = `loop`
    loop {
        println!("Enter your guess:");
        
        // Declare variables with `let`
        // mutable object because we'll be changing it from io
        let mut guess = String::new();

        // standard input
        //      read the line in terminal
        //      raise message if anything goes wrong for some reason
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Rust has type hints!!!!
        // Except they're not type hints, they're the actual type.
        // u32 is unsigned 32 bit int
        //
        // If this were String, e.g., it would break on compile time
        // let guess: String = match guess.trim().parse() {
        let guess: u32 = match guess.trim().parse() {
            // Now this is interesting syntax!
            // Match takes the result from whatever's passed to it,
            //  and does one of the following depending on the return
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            },
        };

        println!("You guessed: {}", guess);

        // Same match logic as before,
        // this is basically a case {}
        // but with some interesting functional-esque syntax
        match guess.cmp(&correct) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
