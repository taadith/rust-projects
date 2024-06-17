use std::io;
use std::cmp::Ordering;

fn main() {
    let cin = io::stdin();

    // reading in n
    let mut n: String = String::new();
    cin.read_line(&mut n).expect("failed to read line for n");
    let n: usize = n.trim().parse().expect("expected an unsigned integer");

    let mut v: Vec<(String, String, String)> = Vec::new();
    for _ in 0..n {
        let mut input: String = String::new();
        cin.read_line(&mut input).expect("failed to read input for BirthDate");
        let bd_info: Vec<String> = input.split(" ")
            .map(|x| x.trim().parse().expect("not a String"))
            .collect();
        assert!(bd_info.len() == 3);

        let bd_info: (String, String, String) = (
            bd_info[1].clone(),
            bd_info[0].clone(),
            bd_info[2].clone()
        );

        v.push(bd_info);
    }
    assert!(v.len() == n);
    
    // debug:
    println!("b4 sorting: {:?}", v);
    v.sort_by(|a, b| {
        let ordering = a.0.cmp(&b.0);
        if ordering == Ordering::Equal {
            let ordering = a.1.cmp(&b.1);
            if ordering == Ordering::Equal {
                a.2.cmp(&b.2)
            } else {
                ordering
            }
        } else {
            ordering
        }
    });
    assert!(v.len() == n);
    println!("after sorting: {:?}", v);
}