use std::io;
use chrono::prelude::*;
use chrono::TimeDelta;

fn main() {
    let cin = io::stdin();
    let current_utc: DateTime<Utc> = Utc::now();

    let mut input = String::new();
    cin.read_line(&mut input).expect("Failed to read line");
    
    let values: Vec<String> = input.trim()
        .split(' ')
        .map(|s| s.parse().expect("expected a String"))
        .collect();
    assert!(values.len() == 3, "expected input length of 3: <day> <month> <year>");

    let user_day: u32 = values[0].trim()
        .parse()
        .expect("expected an unsigned 32-bit integer");

    let user_month: u32 = values[1].trim()
        .parse()
        .expect("expected an unsigned 32-bit integer");

    let user_year: i32 = values[2].trim()
        .parse()
        .expect("expected a signed 32-bit integer");
    
    let user_utc: DateTime<Utc> = Utc.with_ymd_and_hms(user_year,user_month,user_day,0,0,0)
        .unwrap();

    let utc_delta: TimeDelta = current_utc - user_utc;
    println!("{}", utc_delta.num_days());
}
