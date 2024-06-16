use std::io;

fn main() {
    // reading in float
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    let input: f64 = input
        .trim()
        .parse()
        .expect("please type a float value!");

    println!("{:7.3}", input);
}