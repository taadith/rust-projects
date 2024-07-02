use std::io;

fn main() {
    // reading in user input
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read user input");

    // converting user input to floating 64-bit value
    let input: f64 = input
        .trim()
        .parse()
        .expect("expected a floating number");
    
    // formatting time!
    let formatted_input = format!("{:7.3}", input);
    println!("{formatted_input}");
}