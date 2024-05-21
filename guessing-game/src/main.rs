use std::io;    // for standard user input/output
use rand::Rng;  // Rng trait defines methods for rng
use std::cmp::Ordering; // used for comparing

fn main() {
    println!("guess the number!");

    let secret_num = rand::thread_rng() // gives us rng of local thread
        .gen_range(1..=100);    // [1,100]
    
    loop{
        println!("please input your guess: ");

        let mut guess = String::new();  // mutable/modifiable data

        io::stdin() // returns handle to the std input for terminal
            .read_line(&mut guess)  // gets input from the user and stores in guess
            .expect("failed to read the line"); // checks for successful operation

        // trim removes whitespace from the beginning and end
        // parse converts a String to another type (hint - ": u32")
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("you guessed: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
