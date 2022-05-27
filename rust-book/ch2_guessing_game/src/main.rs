use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    // Type annotation is required here, otherwise Rust doesn't know what kind of int you mean
    let secret_number: u32 = thread_rng().gen_range(0..101);

    println!("Please guess the number!");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // parse is clever - it's able to parse into whatever your output type needs
        // So type annotation is required here again
        let guess: u32 = guess
            .trim()
            .parse()
            .expect("couldn't parse your input as a number");

        // An example of a match statement
        // cmp appears to only exist as a method on a u32 if I use std::cmp::Ordering
        // So this is weird - including a library can change methods on primitive types
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("your guess was too low"),
            Ordering::Greater => println!("your guess was too high"),
            Ordering::Equal => {
                println!("you win!");
                return;
            }
        }
    }
}
