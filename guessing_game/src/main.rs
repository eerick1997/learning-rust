use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Guess the number!");
        println!("Please input your guess");
        // creating a variable
        // new is an associated function in other languages = static
        // Rust can infer variables
        // By default variables are inmutable to make it mutable add `mut`
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Shadowing we're converting from a String to u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // Like a switch case or a When in Kotlin
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            },
        }
    }
}
