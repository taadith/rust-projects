use std::io;    // inclues ability to accept user input

fn main() {
    println!("guess the number!");

    println!("please input your guess");

    // guess is a mutable variable
    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("you guessed: {guess}");
}
