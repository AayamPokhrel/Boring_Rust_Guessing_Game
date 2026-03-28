//import standard I/O library
use std::io;
// import rand library
use rand::RngExt;

//import compare ordering
use std::cmp::Ordering;

fn main() {
    loop {
        println!("Welcome to Multi-Game Boring showdown....");
        println!("Now, choose an option, which game would you like to play?");
        let mut user_game_choice_input: String = String::new();
        println!("1. Guessing game (1-100)");
        println!("2. Scissors!, Paper!, Rock!");
        println!("0.Exit the game");
        io::stdin()
            .read_line(&mut user_game_choice_input)
            .expect("Failed to read user input");
        let user_game_choice: u8 = match user_game_choice_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Failed to parse the number.");
                continue;
            }
        };
        match user_game_choice {
            1 => {
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
                        Ok(num) => {
                            if num < 1 || num > 100 {
                                println!("Valid range of input is only between 1 to 100.");
                                continue;
                            } else {
                                num
                            }
                        }
                        Err(_) => {
                            println!(
                                "Failed to parse number, please input valid number. Try again genius."
                            );
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
                } //Guess game loop
            } // Guess game match
            2 => {
                loop {
                    let play_randomness: u16 = rand::rng().random_range(1..=3000);
                    let play: u8;
                    println!("---Scissors, Paper, Rock---");
                    println!("Beat the randomness and you'll be untouchable by fate!");
                    let play_name: String; // variables are killed if declared inside if block
                    if play_randomness <= 1000 {
                        play = 1; //Scissors
                        play_name = "Scissors".to_string();
                    } else if play_randomness > 1000 && play_randomness <= 2000 {
                        play = 2; //Paper
                        play_name = "Paper".to_string();
                    } else {
                        play = 3; //Rock
                        play_name = "Rock".to_string();
                    }
                    println!("Now, choose your play.");
                    println!("1. Scissors, 2.Paper, 3.Rock");
                    let mut user_play: String = String::new();
                    io::stdin()
                        .read_line(&mut user_play)
                        .expect("Failed to read user input");
                    let user_play: u8 = match user_play.trim().parse() {
                        Ok(num) => {
                            if num < 1 || num > 3 {
                                println!("Please enter valid range of inputs 1 to 3.");
                                continue;
                            } else {
                                num
                            }
                        }
                        Err(_) => {
                            println!("Unable to parse to u8.");
                            continue;
                        }
                    };
                    if user_play == 1 {
                        println!("You choose Scissors.");
                        println!("AI (Aayam Intelligence) choose {play_name}.");
                    } else if user_play == 2 {
                        println!("You choose Paper.");
                        println!("AI (Aayam Intelligence) choose {play_name}.");
                    } else if user_play == 3 {
                        println!("You choose Rock.");
                        println!("AI (Aayam Intelligence) choose {play_name}.");
                    }

                    //Decision if else if run :) idk if rust has switch so if-switch would've been easier
                    /*
                    |----------Decision logic-----------|
                    |   AI  |   Player  |   Decision    |
                    |   1   |   =1      |   eq          |
                    |   1   |   <2      |   AI          |
                    |   1   |   <3      |   Player      |
                    |   2   |   >1      |   Player      |
                    |   2   |   =2      |   eq          |
                    |   2   |   <3      |   AI          |
                    |   3   |   >1      |   AI          |
                    |   3   |   >2      |   Player      |
                    |   3   |   =3      |   eq          |
                    _____________________________________
                    */
                    if play == user_play {
                        println!("Tie Occured! You cannot exit till you win, play again!");
                    } else if play == 1 && user_play == 2
                        || play == 2 && user_play == 3
                        || play == 3 && user_play == 1
                    {
                        println!("AI Won! Play again, Until you win");
                        continue;
                    } else if user_play == 1 && play == 2
                        || user_play == 2 && play == 3
                        || user_play == 3 && play == 1
                    {
                        println!("You won! Play Again?");
                        println!("Press 2 to play again, 1 to exit to game menu or 0 to exit the entire game.");
                        let mut exit_or_play: String = String::new();
                        io::stdin()
                            .read_line(&mut exit_or_play)
                            .expect("Failed to read user data");
                        let exit_or_play: u8 = match exit_or_play.trim().parse() {
                            Ok(num) => num,
                            Err(_) => {
                                println!("Failed to parse to u8.");
                                continue;
                            }
                        };
                        match exit_or_play {
                            2=>{
                                println!("Playing again!");
                                continue;
                            }
                            1=>{
                                println!("Returning to game menu.");
                                break;
                            }
                            0=>{
                                println!("Exiting game!");
                                return;
                            }
                            _=>{
                                println!("Unexpected occured!");
                                continue;
                            }
                        };// exit_or_play match
                    } //else if
                } // SPR loop
            } // SPR match
            0 => {
                println!("Exiting game.");
                return;
            } //exit match
            _ => {
                println!("Invalid input choose again.");
            } //void match
        }; // match first
    } //main game loop 1st
} // main
