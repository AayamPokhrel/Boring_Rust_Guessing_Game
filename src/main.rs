//use standard I/O library
use std::io;

fn main() {
    println!("Guess the number to be rewarded as Genius.");
    
    println!("Please input your guess: ");
    let mut user_guess: String = String::new();
    io::stdin()
        .read_line(&mut user_guess);
    
    println!("You guessed {user_guess}");
}
