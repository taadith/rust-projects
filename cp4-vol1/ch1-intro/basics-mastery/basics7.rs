fn generate_permutations(chars: &mut [char]) {
    fn permute(chars: &mut [char], start: usize) {
        if start == chars.len() {
            println!("{:?}", chars);
        } else {
            for i in start..chars.len() {
                chars.swap(start, i);
                permute(chars, start + 1);
                chars.swap(start, i);
            }
        }
    }

    permute(chars, 0);
}

fn main() {
    let mut alphabet: Vec<char> = ('A'..='C').collect();
    generate_permutations(&mut alphabet);
}