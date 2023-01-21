use rand::Rng;
use std::env;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut min = 1;
    let mut max = 10;
    let error_int_input = "Input not an integer";

    if args.len() == 3 {
        min = args[1].parse::<i32>().expect(error_int_input);
        max = args[2].parse::<i32>().expect(error_int_input);
    }

    println!("Guess the number between {min} and {max}!");

    let secret_number = rand::thread_rng().gen_range(min..=max);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess_number: i32 = guess.trim().parse().expect(error_int_input);

    if guess_number == secret_number {
        println!("Bravo! {secret_number}");
    } else {
        println!("Miss, the secret number: {secret_number}");
    }
}
