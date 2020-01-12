use std::cmp::Ordering;
use rand::Rng;
use rust_utils::io::read_number;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
    
    
        let guess: u32 = match read_number() {
            Ok(num) => num,
            Err(e) => {
                println!("Error {}", e);
                continue
            }
        };
            
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}