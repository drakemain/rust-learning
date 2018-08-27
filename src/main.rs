extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    let mut side = flip_coin();

    loop {
        let guess = get_numeric_input();

        if guess > 2 || guess < 1 {
            println!("You can only select Heads or Tails.");
        }
        else {
            match guess.cmp(&side) {
                std::cmp::Ordering::Equal => println!("Chiggin dinner."),
                _ => println!("You lose.")
            }

            if !play_again_prompt() {
                println!("Exiting.");
                break;
            } else {
                side = flip_coin();
            }
        }
    }
}

fn flip_coin() -> u32 {
    println!("The coin was flipped!\n1: Heads\n2: Tails");

    rand::thread_rng().gen_range(1, 3)
}

fn get_numeric_input() -> u32 {
    let mut input = String::new();

    loop {
        input.clear();

        io::stdin().read_line(&mut input).expect("Readline failed.");

        let input: u32 = match input.trim().parse() {
            Err(_) => {
                println!("Input must be a positive number!");
                continue
            },
            Ok(number) => number
        };

        return input;
    }
}

fn play_again_prompt() -> bool {
    println!("Play again? (Y/n)");

    let mut input = String::new();

    loop {
        println!("Test");
        input.clear();

        io::stdin().read_line(&mut input).expect("Readline failed.");
        
        if input.trim() == "n" || input.trim() == "N" {
            return false;
        }

        if input.trim() == "y" || input.trim() == "Y" || input.trim() == "" {
            return true;
        }
    }
}