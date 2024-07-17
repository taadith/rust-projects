use std::io;

fn generate_subsets(n: usize) {
    let set: Vec<usize> = (1..=n).collect();
    let subset_count: usize = 1 << n;
    
    for i in 0..subset_count {
        print!("{{");
        for j in 0..n {
            if i & (1 << j) != 0 {
                print!(" {:?}", set[j]);
            }
        }
        println!(" }}");
    }
}

fn main() {
    let cin = io::stdin();

    let mut n: String = String::new();
    cin.read_line(&mut n).expect("failed to read line for n");
    let n: usize = n.trim().parse().expect("expected an unsigned integer");

    generate_subsets(n);
}