//import standard I/O library
use std::io;
// inport rand library
use rand::RngExt;

//import compare ordering
use std::cmp::Ordering;

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

    match user_guess.cmp(&real_generated_number) {
        Ordering::Equal => println!("You won! You are officially genius."),
        Ordering::Less => println!("Try hard genius, your guess is too small."),
        Ordering::Greater => println!("Try hard genius, your guess is too big."),
    }
}
