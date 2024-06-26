use std::io;

fn bin_exp(a: &usize, b: &usize) -> usize {
    let mut res: usize = 1;

    let mut base: usize = *a;
    let mut exp: usize = *b;
    while exp > 0 {
        if exp & 1 > 0 {
            res *= base;
        }
        base *= base;
        exp >>= 1;
    }
    res
}

fn generate_pwrset(arr: &mut [usize]) {
    let base: usize = 2;
    let exp: usize = arr.len();
    let pwrset_size: usize = bin_exp(&base, &exp);

    for i in 0..pwrset_size {
        print!("{{");
        for j in 0..arr.len() {
            if (i >> j) & 1 == 1 {
                print!(" {} ", arr[j]);
            }
        }
        println!("}}");
    }
}

fn main() {
    let cin = io::stdin();

    // reading in n:
    let mut n: String = String::new();
    cin.read_line(&mut n).expect("failed to read line for n");
    let n: usize = n.trim().parse().expect("expected an unsigned integer for n");

    // reading in nums
    let mut input: String::new();
    cin.read_line(&mut input).expect("failed to read line for input");
    let mut nums: Vec<usize> = input.split(" ")
        .map(|x| x.trim().parse().expect("not a usize"))
        .collect();
    
    generate_pwrset(& mut nums);
}