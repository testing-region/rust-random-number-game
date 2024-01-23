extern crate rand;

use colorz::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Type 'exit' to close the program\n");
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess: ");
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
                    break;
                } else {
                    println!("{}", "Invalid input: Please enter a number!!!\n".yellow());
                    continue;
                }
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
            Ordering::Greater => println!("{}", "Too big!".red()),
        }

        println!();
    }
}
