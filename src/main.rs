extern crate rand;

use colorz::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("Guess the number!");
    println!("You have 5 tries to get the number.");
    println!("Type 'exit' to close the program");

    let mut score = 0;
    let mut games_count = 0;
    let mut option = String::new();

    loop {
        println!();
        // count game statistics
        score += play();
        games_count += 1;
        println!("\nCurrent score: {}", score);

        // ask if user wants to continue playing
        print!("Do you want to play again? [y/n]: ");
        std::io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read input");

        let option = option.trim();

        if option == "y" {
            continue;
        }

        break;
    }

    println!("\nYou played {} game(s)", games_count);
    println!("Total score: {}", score);
}

fn play() -> u32 {
    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut tries = 0;

    loop {
        if tries == 5 {
            println!("The secret number was {}", secret_number);
            return 0;
        }

        print!("Please input your guess: ");
        std::io::stdout().flush().unwrap();
        let mut guess = String::new();

        // read and process input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        // convert to integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if guess.trim() == "exit" {
                    return 0;
                }
                println!("{}", "Invalid input: Please enter a number!!!\n".yellow());
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                return 1;
            }
            Ordering::Greater => println!("{}", "Too big!".red()),
        }

        println!();
        tries += 1;
    }
}
