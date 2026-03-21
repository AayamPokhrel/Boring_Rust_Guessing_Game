//import standard I/O library
use std::io;
// inport rand library
use rand::RngExt;

fn main() {
    println!("Guess the number to be rewarded as Genius.");
    
    println!("Please input your guess: ");
    let mut user_guess: String = String::new();
    io::stdin()
        .read_line(&mut user_guess)
        .expect("Failed to read user input");
    let real_generated_number: u8 = rand::rng().random_range(1..=100);
    let user_guess: u8 = user_guess.trim().parse().expect("Failed to parse!");
    println!("Generated number: {real_generated_number}");
    println!("You guessed {user_guess}");
}
