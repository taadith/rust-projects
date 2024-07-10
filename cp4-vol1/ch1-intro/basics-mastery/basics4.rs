use std::io;

fn main() {
    let cin = io::stdin();

    let mut n: String = String::new();
    cin.read_line(&mut n).expect("failed to read line");
    let n: usize = n.trim()
        .parse()
        .expect("expected an unsigned integer");

    let mut values: String = String::new();
    cin.read_line(&mut values).expect("failed to read line");
    let mut values: Vec<usize> = values.trim()
        .split(' ')
        .map(|x| x.parse().expect("expected an unsigned integer"))
        .collect();
    assert!(values.len() == n, "values.len() =/= n");

    println!("unsorted values: {:?}", values);
    values.sort();
    println!("sorted values: {:?}", values);
}