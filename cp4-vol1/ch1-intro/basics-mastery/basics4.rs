use std::io;

fn main() {
    let cin = io::stdin();

    // reading in n
    let mut n: String = String::new();
    cin.read_line(&mut n).expect("failed to read line for n");
    let n: usize = n.trim().parse().expect("expected an unsigned integer");

    // input
    let mut input: String = String::new();
    cin.read_line(&mut input).expect("failed to read line for input");
    let mut v: Vec<usize> = input.split(" ")
        .map(|x| x.parse().expect("not an integer"))
        .collect();
    assert!(v.len() == n);

    v.sort();
    println!("{:?}", v);
}