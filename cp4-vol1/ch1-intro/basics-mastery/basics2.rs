use std::io;

fn main() {
    let cin = io::stdin();
    const PI: f64 = 3.141592653589793238462643;

    let mut n: String = String::new();
    cin.read_line(&mut n)
        .expect("failed to read n");
    let n: usize = n
        .trim()
        .parse()
        .expect("expected an unsigned integer that could fit in a byte");
    assert!(n < 16, "n = {} >= 16", n);
    
    println!("{:.*}", n, PI);
}