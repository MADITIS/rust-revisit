use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let scret_number = rand::thread_rng().gen_range(1..=100);

    println!("please input your guess: {scret_number}");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("your guess: {guess}")
}
