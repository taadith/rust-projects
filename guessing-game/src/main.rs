use std::io;    // inclues ability to accept user input

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // guess is a mutable variable
    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("you guessed: {guess}");
}
