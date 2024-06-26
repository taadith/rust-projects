use std::io;

// heap algorithm for permutations
fn generate_permutations(arr: &mut [char], start: usize) {
    if start == arr.len() - 1 {
        println!("{:?}", arr.iter().collect::<String>());
    } else {
        for i in start..arr.len() {
            arr.swap(start, i);
            generate_permutations(arr, start + 1);
            arr.swap(start, i);
        }
    }
}

fn main() {
    let cin = io::stdin();

    // reading in n
    let mut n: String = String::new();
    cin.read_line(&mut n).expect("failed to read line for n");
    let n: usize = n.trim().parse().expect("expected an unsigned integer");

    // reading in letters
    let mut input: String = String::new();
    cin.read_line(&mut input).expect("failed to read line for input");
    let mut letters: Vec<char> = input.split(" ")
        .map(|x| x.trim().parse().expect("not a char"))
        .collect();
    assert!(n == letters.len());

    generate_permutations(&mut letters, 0);
}