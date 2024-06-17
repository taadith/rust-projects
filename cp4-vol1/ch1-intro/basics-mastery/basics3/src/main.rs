use std::io;
use chrono::prelude::*;

fn main() {
    // will receive day, month, year
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");

    // [month, day, year]
    let prev_date: Vec<String> = input.split(" ")
        .map(|x| x.parse().expect("not a String"))
        .collect();

    let prev_month: u32 = prev_date[0].trim().parse().unwrap();
    let prev_day: u32 = prev_date[1].trim().parse().unwrap();
    let prev_year: i32 = prev_date[2].trim().parse().unwrap();

    let prev_dt = Utc.with_ymd_and_hms(prev_year, prev_month, prev_day, 0, 0, 0).unwrap();
    let curr_dt = Utc::now();

    let diff_dt = curr_dt - prev_dt;
    print!("Weekday of Inputted Date: {}\n", prev_dt.weekday());
    println!("Days of Difference: {}", diff_dt.num_days());
}
