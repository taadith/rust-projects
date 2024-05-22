fn fahr_to_cels(fahr_degrees: f64) {
    let cels_degrees: f64 = (fahr_degrees - 32.0) / 1.8;
    println!("{fahr_degrees} degrees F = {cels_degrees} degrees C");
}

fn cels_to_fahr(cels_degrees: f64) {
    let fahr_degrees: f64 = (cels_degrees * 1.8) + 32.0;
    println!("{cels_degrees} degrees C = {fahr_degrees} degrees F");
}

fn nth_fib_num(n: u64) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 || n == 2 {
        return 1;
    } else {
        let mut prev: u64 = 0;
        let mut curr: u64 = 1;
        for _x in 0..n-1 {
            let temp: u64 = curr;
            curr += prev;
            prev = temp;
        }
        return curr;
    }
}

fn twelve_days_of_christmas() {
    let ordinal_adverbs: [&str; 12] = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelvth"
    ];
    let presents_on_days: [&str; 12] = [
        "partridge in a pear tree.",
        "Two turle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans-a-swimming,",
        "Eights maids-a-milking,",
        "Nine ladies dancing,",
        "Ten lords-a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,"
    ];

    for day in 1..=12 {
        println!("On the {} day of Christmas my true love sent to me", ordinal_adverbs[day - 1]);
        for presents in (1..=day).rev() {
            if day == 1 {
                println!("A {}", presents_on_days[presents - 1]);
            } else if presents == 1 {
                println!("And a {}", presents_on_days[presents - 1]);
            } else {
                println!("{}", presents_on_days[presents - 1]);
            }
        }
        println!();
    }
}

fn main() {
    let age: i32 = 20;

    if age < 21 {
        println!("you cannot drink alcohol!");
    } else {
        println!("you can drink alcohol!")
    }
    println!();

    let condition: bool = false;
    // both values need to be same type
    let number: i32 = if condition {5} else {6};
    println!("number = {number}");
    println!();

    let mut counter: u8 = 0;
    
    let result: u8 = loop {
        counter += 1;
        
        if counter == 10 {
            // break can take an optional expression as an argument
            break counter * 2;
        }
    };
    println!("result = {result}");
    println!();

    counter = 0;
    'counting_up: loop {
        println!("counter = {counter}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        counter += 1;
    }
    println!("end value of counter = {counter}");
    println!();

    let mut number: i32 = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");
    println!();

    // let a: [i32; 5] = [10, 20, 30, 40, 50];
    // let mut index: usize = 0;
    // while index < 5 {
    //     println!("the value at index {} of a is {}", index, a[index]);
    //     index += 1;
    // }
    // for element in a {
    //     println!("value is {element}");
    // }
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
    println!();

    fahr_to_cels(212 as f64);
    cels_to_fahr(100 as f64);
    println!();

    for n in 0..13 {
        let fib_n: u64 = nth_fib_num(n);
        println!("fib({n}) = {fib_n}");
    }
    println!();

    twelve_days_of_christmas()
}
