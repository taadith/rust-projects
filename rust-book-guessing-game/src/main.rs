use std::io;    // includes ability to accept user input
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("guess the number!");

    let secret_num: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("please input your guess");

        // guess is a mutable variable
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("{guess} is too small"),
            Ordering::Greater => println!("{guess} is too big"),
            Ordering::Equal => {
                println!("{guess} is the correct number, you win!!!");
                break;
            }
        }
    }
}
