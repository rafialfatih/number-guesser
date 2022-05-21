use std::io::{stdin, stdout, Write};
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    loop {
        let mut play_again = String::new();
        
        println!("\x1B[2J");
        println!("{}", "Guess The Secret Number!".cyan().bold());

        println!("\nChoose the difficulty!\n1. Easy\n2. Medium\n3. Hard\n");

        let mut difficulty = String::new();
        stdin()
            .read_line(&mut difficulty)
            .expect("Failed to read line");

        let secret_number = rand::thread_rng().gen_range(1, 100);

        let mut life: u32;

        if difficulty.trim() == "1" {
            life = 15;
        } else if difficulty.trim() == "2" {
            life = 10;
        } else if difficulty.trim() == "3" {
            life = 5;
        } else {
            println!("{}", "Invalid input! Please try again.".red());
            continue;
        }
        
        println!("\x1B[2J");
    
        loop {
            println!("\nYou have {} life(s)!", life);
            print!("\nInput your number: ");
            stdout().flush().unwrap();

            let mut guess = String::new();

            stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("{}", "Input must a number!".red().bold());
                    continue;
                },
            };

            println!("You guessed: {}", guess);
            
            match guess.cmp(&secret_number) {
                Ordering::Less => {
                    println!("{}", "Too small!".yellow());
                    life -= 1;
                },
                Ordering::Greater => {
                    println!("{}", "Too big!".red());
                    life -= 1;
                },
                Ordering::Equal => {
                    println!("{}", "\nCongratulations, You win!".green());
                    println!("The secret number is {}", secret_number);
                    break
                },
            }

            if life == 0 {
                println!("{}", "\nYou lose!".red());
                println!("The secret number is {}", secret_number);
                break
            }
        }

        print!("\nDo you want to play again? (y/n) ");
        stdout().flush().unwrap();

        stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line");

        if play_again.trim() == "n" {
            break;
        }
    }
}
