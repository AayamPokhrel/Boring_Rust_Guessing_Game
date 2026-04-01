//imports
use rand::RngExt; //random generation lib
use std::cmp::Ordering; // Compare Ordering lib

fn parse_read() -> u8 {
    // a function to read user's input and parse it.
    let mut user_choice: String = String::new();
    std::io::stdin() //std to avoid whole standard lib import
        .read_line(&mut user_choice)
        .expect("Failed to read user data.");
    let user_choice: u8 = match user_choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Failed to parse the number. Enter valid input.");
            0 //fallback value for u8 : later setting conditions for returned 0
        }
    };
    return user_choice;
}

fn guess_game() {
    //Guessing game function
    println!("\n---Guessing Game---");
    println!("Guess the number to be rewarded as Genius.");
    let real_generated_number: u8 = rand::rng().random_range(1..100);
    loop {
        let user_guess: u8 = loop {
            println!("\nPlease input your guess(1-100):");
            let input = parse_read();
            match input {
                1..=100 => break input,
                _ => println!("\nInvalid input, Range from 1-100 only."),
            };
        }; // input loop
        println!("\nYou guessed {user_guess}");

        match user_guess.cmp(&real_generated_number) {
            Ordering::Equal => {
                println!("\nYou won! You are officially genius. Play Again?");
                println!(
                    "\nPress 2 to play again, 1 to exit to game menu or 0 to exit the entire game."
                );
                let exit_or_play: u8 = loop {
                    let input = parse_read();
                    match input {
                        0..=2 => break input,
                        _ => println!("Invalid input, Range from 0,1,2 only."),
                    };
                };
                match exit_or_play {
                    2 => {
                        println!("Playing again!");
                        continue;
                    }
                    1 => {
                        println!("Returning to game menu.");
                        break;
                    }
                    0 => {
                        println!("Exiting game!");
                        return;
                    }
                    _ => {
                        println!("Unexpected occured!");
                        continue;
                    }
                }; // exit_or_play match
            }
            Ordering::Less => println!("\nTry again, your guess is too small."),
            Ordering::Greater => println!("\nTry again, your guess is too big."),
        };
    } //main game loop
}

fn scissors_paper_rock() {
    // Scissors, Paper, Rock function
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
    println!("\n---Scissors, Paper, Rock---");
    println!("Beat the randomness and you'll be untouchable by fate!");
    loop {
        let ai_play: u8 = rand::rng().random_range(1..=3); //use simple random range.
        let ai_play_name: String = match ai_play {
            1 => "Scissors",
            2 => "Paper",
            3 => "Rock",
            _ => {
                println!("Unexpected occured!");
                continue;
            }
        }
        .to_string(); //match ai_play
        println!("\nNow, choose your play.");
        println!("\n1. Scissors, 2.Paper, 3.Rock");
        let user_play: u8 = loop {
            let input: u8 = parse_read();
            match input {
                1..=3 => break input,
                _ => println!("Invalid input, Range from 1,2,3 only."),
            };
        };
        match user_play {
            1 => println!("\nYou choose Scissors."),
            2 => println!("\nYou choose Paper."),
            3 => println!("\nYou choose Rock."),
            _ => println!("\nFailed to read user play."),
        }
        println!("AI(Aayam Intelligence choose) {ai_play_name}");
        //Decision
        match (ai_play, user_play) {
            (ai, player) if ai == player => {
                println!("\nTie Occured! You cannot exit till you win, play again!");
                continue;
            }
            (1, 2) | (2, 3) | (3, 1) => {
                println!("\nAI Won! Play again, Until you win");
                continue;
            }
            (2, 1) | (3, 2) | (1, 3) => {
                println!("\nYou won! Play Again?");
                println!(
                    "\nPress 2 to play again, 1 to exit to game menu or 0 to exit the entire game."
                );
                let exit_or_play: u8 = loop {
                    let input = parse_read();
                    match input {
                        0..=2 => break input,
                        _ => println!("Invalid input, Range from 0,1,2 only."),
                    };
                };
                match exit_or_play {
                    2 => {
                        println!("Playing again!");
                        continue;
                    }
                    1 => {
                        println!("Returning to game menu.");
                        break;
                    }
                    0 => {
                        println!("Exiting game!");
                        return;
                    }
                    _ => {
                        println!("Unexpected occured!");
                        continue;
                    }
                }; // exit_or_play match
            }
            (_, _) => {
                println!("Failed evaluating decision please try again.");
                continue;
            }
        };
    }
}

fn main() {
    // main function where our code starts
    loop {
        println!("\n-----> MegaMind Games <-----\n");
        println!("Please choose a game you would like to play!");
        println!("1.Guessing game (From 1 to 100).");
        println!("2.Scissors, Paper, Rock.");
        println!("0. Exit the game.\n");
        println!("Input your choice: ");
        let menu_choice: u8 = parse_read();
        match menu_choice {
            1 => guess_game(),
            2 => scissors_paper_rock(),
            0 => return,
            _ => {
                println!("Something went wrong!\n");
                continue;
            }
        }; // menu_choice match
    }
}
