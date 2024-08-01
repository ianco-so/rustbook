use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number: u16 = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");
    let mut guess = String::new();
    loop {
        guess.clear();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        if guess.trim() == "quit" || guess.trim() == "exit" || guess.trim() == "q" {
            println!("Goodbye!");
            break;
        }
        let guess: u16 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number between 1 and 100!");
                continue;
            }
        };
        print!("You guessed {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!(" But it's too small!"),
            Ordering::Greater => println!(" But it's too big!"),
            Ordering::Equal => {
                println!(" And you win!");
                break;
            }
        }
    }
}
