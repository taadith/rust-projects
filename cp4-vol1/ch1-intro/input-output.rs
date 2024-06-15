use std::io;

fn main() {
    let mut input: String = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{} bytes read for input: {}", n, input);
        }
        Err(error) => {
            println!("error: {error}");
        }
    }
}