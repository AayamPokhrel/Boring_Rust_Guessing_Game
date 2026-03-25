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
    } //Guess game loop
    }, // Guess game match
    Ok(2) => {
    loop{
    let play_randomness:u16 = ramd::rng().random_range(1..=1000);
    let mut play:u8;
    println!("---Scissors, Paper, Rock---");
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
        println!("AI (Aayam Intelligence) choose {play_name}.");
    }
    else if(user_play==2){
        println!("You choose Paper.");
        println!("AI (Aayam Intelligence) choose {play_name}.");
    }
    else if(user_play==3){
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
    if(play==user_play){
    println!("Tie Occured! You cannot exit till you win, play again!");
    }
    else if (user_play ==1 && play < user_play){
    println!("AI Won! Play again, Until you win");
    continue;
    }
    else if (user_play ==2 && play < user_play){
    println!("AI Won! Play again, Until you win");
    continue;
    }
    else if (user_play ==3 && play < user_play){
    println!("AI Won! Play again, Until you win");
    continue;
    }
    else{
    println!("You won! Play Again?");
    println!("Press 1 to play again or 0 to exit");
        let mut exit_or_play:u8;
        io::stdin()
            .read_line(&mut exit_or_play);
            .expect("Failed to read user data");
        if(exit_or_play==1)
        {
            continue;
        }
        else if(exit_or_play==0){
        break;
        }
        else{
        println!("Enter valid input, Crashed..."); // Will set input handleling later
        }
    }
    
} // SPR loop
} // SPR match
} // match first
} // main
