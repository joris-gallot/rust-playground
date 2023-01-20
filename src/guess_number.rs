use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number between 1 and 10!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess_number: i32 = guess.trim().parse().expect("Input not an integer");

    if guess_number == secret_number {
        println!("Bravo! {secret_number}");
    } else {
        println!("Miss, the secret number: {secret_number}");
    }
}
