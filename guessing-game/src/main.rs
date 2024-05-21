use std::io;    // for standard user input/output

fn main() {
    println!("guess the number!");

    println!("please input your guess: ");

    let mut guess = String::new();  // mutable/modifiable data

    io::stdin() // returns handle to the std input for terminal
        .read_line(&mut guess)  // gets input from the user and stores in guess
        .expect("failed to read the line"); // checks for successful operation

    println!("you guessed: {guess}");
}
