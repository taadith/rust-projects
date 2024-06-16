use std::io;

fn main() {
    const PI: f64 = 3.14159265358979323846264338327950288419;

    // reading in integer
    let mut digits_of_pi = String::new();

    io::stdin()
        .read_line(&mut digits_of_pi)
        .expect("failed to read line");

        let digits_of_pi: usize = digits_of_pi
        .trim()
        .parse()
        .expect("please type an integer value!");

    println!("{0:.1$}", PI, digits_of_pi);
}