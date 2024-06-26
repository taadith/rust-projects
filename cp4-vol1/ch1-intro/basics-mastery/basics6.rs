use std::io;

fn main() {
    let cin = io::stdin();
    
    let mut l: String = String::new();
    cin.read_line(&mut l).expect("l: failed to read line");
    let l: usize = l.trim().parse().expect("expected an unsigned integer");

    let mut input: String = String::new();
    cin.read_line(&mut input).expect("input: failed to read line");
    let v: Vec<usize> = input.split(" ")
        .map(|x| x.trim().parse().expect("not an unsigned integer"))
        .collect();
    assert!(v.len() == l);

    let mut val: String = String::new();
    cin.read_line(&mut val).expect("val: failed to read line");
    let val: usize = val.trim().parse().expect("expected an unsigned integer");

    match v.binary_search(&val) {
        Ok(x) => {
            println!("value found at index {x}");
        }
        Err(_) => {
            println!("value not found");
        }
    }
}