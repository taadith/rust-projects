use std::io;    // for standard user input/output
use rand::Rng;  // Rng trait defines methods for rng
use std::cmp::Ordering; // used for comparing

fn main() {
    println!("guess the number!");

    let secret_num = rand::thread_rng() // gives us rng of local thread
        .gen_range(1..=100);    // [1,100]

    println!("secret number: {secret_num}");

    println!("please input your guess: ");

    let mut guess = String::new();  // mutable/modifiable data

    io::stdin() // returns handle to the std input for terminal
        .read_line(&mut guess)  // gets input from the user and stores in guess
        .expect("failed to read the line"); // checks for successful operation

    println!("you guessed: {guess}");
}
