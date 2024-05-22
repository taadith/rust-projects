fn main() {
    another_function(5);
    print_labeled_measurement(5,'h');

    let y = {
        let x = 3;
        println!("x = {x}");
        x + 1
    };
    // doesn't work bc x is out of scope
    // println!("y = x + 1 = {x} + 1 = {y}");
    println!("y = x + 1 = 3 + 1 = {y}");

    let x = return_five();
    println!("x = {x}");

    let x = plus_one(x);
    println!("x = {x}")
}

// required type annotations for parameters
fn another_function(x: i32) {
    println!("value of argument passed to parameter: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("the measurement is {value}{unit_label}")
}

fn return_five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
