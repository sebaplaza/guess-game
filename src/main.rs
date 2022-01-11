use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!! 🥳");
    println!("Please input your guess");
    let mut rng = thread_rng();
    let mut tries = 0;
    let secret_number = rng.gen_range(1..101);
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("unable to read input");
        tries = tries + 1;
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input your guess");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low !"),
            Ordering::Greater => println!("Too high !"),
            Ordering::Equal => {
                println!("You won in {} tries!!!! 🎉 😘 💩", tries);
                break;
            }
        }
    }
}
