use std::io;

fn main() {
    let cin = io::stdin();

    let mut l: String = String::new();
    cin.read_line(&mut l).expect("failed to read line for l");
    let l: usize = l.trim().parse().expect("expected an unsigned integer");

    let mut items: String = String::new();
    cin.read_line(&mut items).expect("failed to read line for items");
    let items: Vec<usize> = items.trim()
        .split(' ')
        .map(|x| x.parse().expect("expected an unsigned integer"))
        .collect();
    assert!(items.len() == l, "items.len() =/= l");

    let mut v: String = String::new();
    cin.read_line(&mut v).expect("failed to read line for v");
    let v: usize = v.trim().parse().expect("expected an unsigned integer");

    match items.binary_search(&v) {
        Ok(i) => {
            println!("value {} found at index {}", v, i);
        },
        Err(_) => {
            println!("value {} not found", v);
        }
    };

}