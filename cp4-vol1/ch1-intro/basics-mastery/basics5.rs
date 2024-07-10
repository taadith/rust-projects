use std::io;

fn main() {
    let cin = io::stdin();

    let mut n: String = String::new();
    cin.read_line(&mut n).expect("failed to read line for n");
    let n: usize = n.trim().parse().expect("expected an unsigned integer");

    let mut birthdays: Vec<(usize, usize, usize)> = Vec::new();
    for _ in 0..n {
        let mut birthday: String = String::new();
        cin.read_line(&mut birthday).expect("failed to read line for values");
        let birthday: Vec<usize> = birthday.trim()
            .split(' ')
            .map(|x| x.parse().expect("expected an unsigned integer"))
            .collect();
        assert!(birthday.len() == 3, "incorrect input, please enter in format: <DD> <MM> <YYYY>");
        let birthday: (usize, usize, usize) = (birthday[1], birthday[0], birthday[2]);
        birthdays.push(birthday);
    }
    assert!(birthdays.len() == n, "birthdays.len() =/= n");
    
    println!("before sorting: {:?}", birthdays);
    birthdays.sort_by(|a, b| {
        a.0.cmp(&b.0)
            .then(a.1.cmp(&b.1))
            .then(a.2.cmp(&b.2))
    });
    println!("after sorting: {:?}", birthdays);

}