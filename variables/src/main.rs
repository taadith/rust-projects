fn main() {
    let x = 5;  // by default, immutable
    println!("original value of x: {x}");

    // cannot assign twice to immutable variable
    // x = 6;
    // println!("new value of x: {x}");

    let mut y = 6;  // now by default, mutable
    println!("original value of y: {y}");
    y = 7;
    println!("new value of y: {y}");

    // must always annotate type of constant
    const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // let keyword effectively creates a new variable
    let spaces = "    ";
    println!("spaces: {spaces}");
    let spaces = spaces.len();
    println!("spaces: {spaces}");

    // results in error b/c expects spaces.len() to be &str, not usize
    // let mut spaces = "    ";
    // let spaces = spaces.len();

    let _floatx = 2.0; // by default, f64
    let _floaty = 3.0; // explicit annotation of f32

    let quot = 56.7 / 32.2;
    println!("quot: {quot}");

    let tup = (500, 6.4, 1);
    let (tx, ty, tz) = tup;
    println!("tup: ({tx}, {ty}, {tz})");
}