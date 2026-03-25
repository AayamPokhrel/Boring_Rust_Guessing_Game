//import standard I/O library
use std::io;
// import rand library
use rand::RngExt;

//import compare ordering
use std::cmp::Ordering;

fn main() {

    println!("Welcome to Multi-Game Boring showdown....");
    println!("Now, choose an option, which game would you like to play?");
    let mut user_game_choice: u8;
    println!("1. Guessing game (1-100)");
    println!("2. Scissors!, Paper!, Rock!");
    io::stdin()
        .read_line(&mut user_game_choice)
        .expect("Failed to read user input");
        match user_game_choice{
        Ok(1) => {
        println!("---Guessing Game---");
    println!("Guess the number to be rewarded as Genius.");

    let real_generated_number: u8 = rand::rng().random_range(1..=100);
    loop {
        println!("Please input your guess(1-100): ");
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
    },
    Ok(2) => {
    let play_randomness:u16 = ramd::rng().random_range(1..=1000);
    let mut play:u8;
    println!("---Scissors, Paper, Rock");
    println!("Beat the randomness and you'll be untouchable by fate!");
    if(play_randomness<=1000){
        play=1; //Scissors
        let play_name:String = "Scissors";
    }
    else if (play_randomness>1000 && play_randomness<=2000){
        play=2; //Paper
        let play_name:String = "Paper";
        }
    else{
        play=3; //Rock
        let play_name:String = "Rock";
        }
        println!("Now, choose your play.");
        println!("1. Scissors, 2.Paper, 3.Rock");
        let mut user_play:u8;
        io::stdin()
            .read_line(&mut user_play)
                .expect("Failed to read user input");
        
        if(user_play==1){
        println!("You choose Scissors.");
        println!("AI(Aayam Intelligence choose 
    }
    else if(user_play==2){
        println!("You choose Paper.");
    }
    else if(user_play==3){
        println!("You choose Rock.");
    }
}
}
