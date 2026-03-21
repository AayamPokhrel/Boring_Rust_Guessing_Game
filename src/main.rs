//import standard I/O library
use std::io;
// import rand library
use rand::RngExt;

//import compare ordering
use std::cmp::Ordering;

fn main() {
    println!("Guess the number to be rewarded as Genius.");

    let real_generated_number: u8 = rand::rng().random_range(1..=100);
    loop {
        println!("Please input your guess: ");
        let mut user_guess: String = String::new();

        io::stdin()
            .read_line(&mut user_guess)
            .expect("Failed to read user input");

        let user_guess: u8 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Failed to parse number, please input valid number. Try again genius.");
                continue;
            }
        };
        println!("You guessed {user_guess}");

        match user_guess.cmp(&real_generated_number) {
            Ordering::Equal => {
                println!("You won! You are officially genius.");
                break;
            }
            Ordering::Less => println!("Try hard genius, your guess is too small."),
            Ordering::Greater => println!("Try hard genius, your guess is too big."),
        }
    }
}
