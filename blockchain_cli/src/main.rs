#[macro_use]
extern crate serde_derive;

use std::io;
use std::io::{stdin, stdout, Write};
use std::process;

mod blockchain;

fn main() {
    let mut miner_address = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("Input a miner address: ");
    stdout().flush().expect("Could not read input");
    stdin()
        .read_line(&mut miner_address)
        .expect("Could not read input");

    print!("Input a difficulty level: ");
    stdout().flush().expect("Could not read input");
    stdin()
        .read_line(&mut difficulty)
        .expect("Could not read input");
    let difficulty = difficulty
        .trim()
        .parse::<u32>()
        .expect("Error: Input must be an integer");
    println!("Generating genesis block! ");
    let mut chain = blockchain::Chain::new(miner_address.trim().to_string(), difficulty);

    loop {
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("0) Exit");
        println!("Enter your choice: ");
        io::stdout().flush().expect("Could not read input");
        choice.clear();
        io::stdin()
            .read_line(&mut choice)
            .expect("Could not read input");
        println!();

        match choice.trim().parse().unwrap() {
            0 => {
                println!("Exiting!");
                process::exit(0);
            }
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                println!("Enter a sender address:");
                io::stdout().flush().expect("Could not read input");
                io::stdin()
                    .read_line(&mut sender)
                    .expect("Could not read input");
                println!("Enter a receiver address:");
                io::stdout().flush().expect("Could not read input");
                io::stdin()
                    .read_line(&mut receiver)
                    .expect("Could not read input");
                println!("Enter an amount:");
                io::stdout().flush().expect("Could not read input");
                io::stdin()
                    .read_line(&mut amount)
                    .expect("Could not read input");

                let result = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap(),
                );

                if result {
                    println!("Transaction added");
                } else {
                    println!("Transaction failed");
                }
            }
            2 => {
                println!("Generating block");
                let result = chain.generate_new_block();

                if result {
                    println!("Block generated successfully");
                } else {
                    println!("Block generation failed");
                }
            }
            3 => {
                let mut new_difficulty = String::new();
                print!("Enter a new difficulty level: ");
                io::stdout().flush().expect("Could not read input");
                io::stdin()
                    .read_line(&mut new_difficulty)
                    .expect("Could not read input");
                let result = chain.update_difficulty(new_difficulty.trim().parse().unwrap());
                if result {
                    println!("Updated difficulty level");
                } else {
                    println!("Failed updated difficulty");
                }
            }
            4 => {
                let mut new_reward = String::new();
                print!("Enter a new reward: ");
                io::stdout().flush().expect("Could not read input");
                io::stdin()
                    .read_line(&mut new_reward)
                    .expect("Could not read input");
                let result = chain.update_reward(new_reward.trim().parse().unwrap());

                if result {
                    println!("Updated reward");
                } else {
                    println!("Failed to update reward");
                }
            }
            _ => println!("\tinvalid option please retry\t"),
        }
    }
}
